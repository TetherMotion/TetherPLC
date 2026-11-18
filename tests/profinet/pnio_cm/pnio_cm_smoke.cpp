#include <tplc/host/PnioCm.hpp>
#include <tplc/host/Dcerpc.hpp>
#include <cstdio>
#include <cstdlib>
#include <cstring>
using namespace tplc::host;
static void puid(const char*n,const std::array<uint8_t,16>&u){printf("%s=",n);for(auto b:u)printf("%02x",b);printf("\n");}
int main(int argc,char**argv){
    const char* port = argc>1?argv[1]:"34964";
    const uint16_t uport = uint16_t(atoi(port));
    tplc::DcRpcLink link;
    if(!link.open("127.0.0.1",uport)){printf("link open fail\n");return 1;}
    PnioArClient ar;
    if(!ar.open(link,"127.0.0.1",uport)){printf("ar open fail\n");return 1;}

    // build Connect blocks
    PnioArConfig cfg;
    cfg.deviceObjectUuid = pnioObjectUuid(1,2,0x0493); // dea00000-...-000100020493
    cfg.activityUuid = {0xae,0xa1,0xac,0xd2,0x32,0x00,0x10,0x10,0xb0,0x58,0xec,0x1c,0x5d,0x61,0xe7,0x3f};
    cfg.ar.arUuid = {0xe5,0xe1,0xae,0xcc,0xb1,0x33,0x4b,0x4d,0xb1,0x87,0xcc,0x68,0xb0,0x21,0x1e,0xd2};
    cfg.ar.sessionKey=0; cfg.ar.arType=1; cfg.ar.activityTimeoutFactor=200;
    cfg.ar.initiatorUdpRtPort=0x8892; cfg.ar.stationName="CPU-01";
    uint8_t mac[6]={0xec,0x1c,0x5d,0x61,0xe7,0x3f}; memcpy(cfg.ar.initiatorMac,mac,6);
    cfg.ar.initiatorObjectUuid = pnioObjectUuid(0,0x0102,0x0493); // controller obj
    cfg.ar.arProperties=0x00000001;

    PnioIocrBlockReq in, out;
    in.iocrType=1; in.reference=1; in.dataLength=12; in.frameId=0x8000;
    in.sendClockFactor=1; in.reductionRatio=32; in.watchdogFactor=0; in.dataHoldFactor=6;
    PnioIocrApi api; api.api=0;
    api.ioData.push_back({0,0x8000,0}); api.iocs.push_back({0,0x8001,12});
    in.apis.push_back(api);
    out=in; out.iocrType=2; out.reference=2; out.frameId=0xffff;
    out.apis[0].ioData[0].frameOffset=0;
    cfg.iocrs={in,out};

    PnioExpectedSubmoduleBlockReq ex; PnioExpectedApi ea;
    ea.api=0; ea.slot=0; ea.moduleIdent=0; ea.moduleProperties=0;
    PnioExpectedSubmodule sm; sm.subslot=0x8000; sm.submoduleIdent=0x00000001;
    sm.properties=0; sm.input=PnioDataDescription{0,1,1};
    ea.submodules.push_back(sm); ex.apis.push_back(ea);
    cfg.expected={ex};
    cfg.alarmCr=PnioAlarmCrBlockReq{};

    // encode connect blocks (same as connect() does)
    std::vector<uint8_t> blocks;
    pnioEncArBlockReq(blocks,cfg.ar);
    for(auto&io:cfg.iocrs)pnioEncIocrBlockReq(blocks,io);
    for(auto&e:cfg.expected)pnioEncExpectedSubmoduleBlockReq(blocks,e);
    pnioEncAlarmCrBlockReq(blocks,cfg.alarmCr);
    printf("connect blocks=%zuB\n",blocks.size());

    if(!ar.connectRaw(cfg.deviceObjectUuid,cfg.activityUuid,cfg.ar.arUuid,
                      blocks.data(),blocks.size(),3000)){
        printf("connect FAIL status=%08x\n",ar.lastStatus());return 1;}
    printf("connect OK key=%u\n",ar.connectRes().sessionKey);
    puid("  ar_uuid",ar.connectRes().arUuid);
    for(auto&i:ar.connectRes().iocrs)printf("  iocr t%u ref%u fid%04x\n",i.iocrType,i.reference,i.frameId);
    printf("  station='%s' alarmMax=%u\n",ar.connectRes().stationName.c_str(),ar.connectRes().alarmMaxDataLen);

    // write F-parameters record
    PnioWriteRecord wr; wr.seq=1; wr.arUuid=cfg.ar.arUuid; wr.api=0; wr.slot=0; wr.subslot=0x8000;
    wr.index=kPnioIndexFParameters;
    PnioFParameters fp; fp.sourceAdd=0x0001; fp.destAdd=0x0002; fp.watchdogTime=100; fp.parCrc=0x1234;
    pnioEncFParameters(wr.data,fp); wr.data.insert(wr.data.end(),{0,0}); // pad to even? F-block is record data as-is
    std::vector<PnioWriteRecord> recs={wr};
    std::vector<uint8_t> wb; pnioEncWriteBlocks(wb,recs);
    std::vector<uint8_t> wres; uint32_t st=0;
    if(!ar.rpcCall(PnioOpnum::Write,wb,wres,st,3000)){printf("write FAIL\n");return 1;}
    printf("write OK status=%08x res=%zuB\n",st,wres.size());

    // I&M0 record read (ReadImplicit — works pre-AR too)
    {
        std::vector<uint8_t> rec;
        if(ar.readRecord(kPnioIndexIm0,0,0x8000,true,rec,3000)){
            PnioIm0View v;
            if(pnioDecIm0(rec.data(),rec.size(),v))
                printf("im0 OK vendor=%04x order='%s' sn='%s' sw=%c.%u.%u.%u\n",
                    v.vendorId,v.orderId.c_str(),v.serialNumber.c_str(),
                    v.swPrefix,v.swFunctional,v.swBugFix,v.swInternal);
            else printf("im0 rec decode FAIL\n");
        } else printf("im0 read FAIL\n");
    }

    // PrmEnd via control opnum
    std::vector<uint8_t> cb; pnioEncControlBlock(cb,pnioPrmEndReq(cfg.ar.arUuid,ar.connectRes().sessionKey));
    std::vector<uint8_t> cres; st=0;
    if(!ar.rpcCall(PnioOpnum::Control,cb,cres,st,3000)){printf("prmend FAIL\n");return 1;}
    PnioControlBlock cdb;
    if(!pnioDecControlBlock(cres.data(),cres.size(),cdb)||cdb.blockType!=kBlkIodControlResPrmEnd){printf("prmend res bad\n");return 1;}
    printf("prmend OK type=%04x cmd=%04x state=%d\n",cdb.blockType,cdb.command,(int)ar.state());

    // service: device AppReady arrives -> respond Done
    for(int i=0;i<20;i++){ if(ar.service(200)) {printf("appready consumed -> state=%d\n",(int)ar.state());break;} }
    printf("final state=%d\n",(int)ar.state());

    // release
    std::vector<uint8_t> rb; pnioEncControlBlock(rb,pnioReleaseReq(cfg.ar.arUuid,ar.connectRes().sessionKey));
    std::vector<uint8_t> rres; st=0;
    if(ar.rpcCall(PnioOpnum::Release,rb,rres,st,3000))printf("release OK status=%08x\n",st);
    else printf("release rpc fail st=%08x\n",st);

    /* I&M codec round-trips (pure codec, no network) */
    int rt=0, rtok=0;
    {
        PnioIm0 im; im.vendorId=0x0493; im.orderId="PNU-RT-200";
        im.serialNumber="SN12345"; im.hardwareRevision=0x0100;
        im.swPrefix='V'; im.swFunctional=1; im.revisionCounter=7;
        im.profileId=0x0001; im.profileSpecificType=0x0001;
        im.imVersionMajor=1; im.imVersionMinor=2; im.supported=0x000E;
        std::vector<uint8_t> enc; pnioEncIm0(enc,im);
        ++rt; PnioIm0View v;
        if(enc.size()==60&&pnioDecIm0(enc.data(),enc.size(),v)&&
           v.vendorId==im.vendorId&&v.orderId=="PNU-RT-200"&&
           v.serialNumber=="SN12345"&&v.hardwareRevision==0x0100&&
           v.swPrefix=='V'&&v.swFunctional==1&&v.revisionCounter==7&&
           v.profileId==1&&v.profileSpecificType==1&&v.supported==0x000E) ++rtok;
        else printf("im0 roundtrip FAIL enc=%zu\n",enc.size());
    }
    {
        PnioImText t; t.a="TAG-FUNC"; t.b="LOC-A1";
        std::vector<uint8_t> e1; pnioEncImRecord(e1,kPnioIndexIm1,t);
        ++rt; PnioImText d;
        if(e1.size()==60&&pnioDecImRecord(e1.data(),e1.size(),kPnioIndexIm1,d)&&
           d.a=="TAG-FUNC"&&d.b=="LOC-A1") ++rtok;
        else printf("im1 roundtrip FAIL\n");
        PnioImText t2; t2.a="2024-01-15";
        std::vector<uint8_t> e2; pnioEncImRecord(e2,kPnioIndexIm2,t2);
        ++rt; PnioImText d2;
        if(e2.size()==22&&pnioDecImRecord(e2.data(),e2.size(),kPnioIndexIm2,d2)&&
           d2.a=="2024-01-15") ++rtok;
        else printf("im2 roundtrip FAIL\n");
        PnioImText t3; t3.a="Device descriptor text";
        std::vector<uint8_t> e3; pnioEncImRecord(e3,kPnioIndexIm3,t3);
        ++rt; PnioImText d3;
        if(e3.size()==60&&pnioDecImRecord(e3.data(),e3.size(),kPnioIndexIm3,d3)&&
           d3.a=="Device descriptor text") ++rtok;
        else printf("im3 roundtrip FAIL\n");
        /* negative: wrong index must fail decode */
        ++rt; PnioImText dn;
        if(!pnioDecImRecord(e1.data(),e1.size(),kPnioIndexIm2,dn)) ++rtok;
        else printf("im wrong-index FAIL\n");
    }
    printf("im roundtrips %d/%d\n",rtok,rt);
    return rtok==rt?0:1;
}
