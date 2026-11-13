/* iec61850_ied_fake.cpp — minimal IEC 61850 IED server for the
 * tplc-iec61850 e2e.  Uses libiec61850's IedServer + CDC model API to
 * expose a small Logical Device / Logical Node tree the client side
 * reads and writes:
 *
 *   domain (LD):    "tetherTestLD"
 *   nodes:          LLN0.Health.stVal  (int32 status, R/W under ST)
 *                   LLN0.Meas.mag.f    (float measurement, R/W under MX)
 *                   LLN0.Name.name     (string, read)
 *
 * Wire-level peer for Iec61850Service — real MMS, not a stub. */
#include <iec61850_server.h>
#include <iec61850_model.h>
#include <iec61850_dynamic_model.h>
#include <iec61850_cdc.h>
#include <hal_thread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>

static volatile int stopFlag = 0;
static void onSig(int) { stopFlag = 1; }

int main(int argc, char** argv) {
    int port = argc > 1 ? atoi(argv[1]) : 1102;

    IedModel* model = IedModel_create("tetherIED");
    LogicalDevice* ld = LogicalDevice_create("tetherTestLD", model);
    LogicalNode* lln0 = LogicalNode_create("LLN0", ld);

    /* Health: enum status (stVal int32) R/W under ST. */
    CDC_ENS_create("Health", (ModelNode*)lln0, 0);
    /* Meas: sampled analog value (instMag.f float) under MX. */
    CDC_SAV_create("Meas", (ModelNode*)lln0, 0, false);
    /* SetPt: analogue setting (setMag.f float) R/W under SP. */
    CDC_ASG_create("SetPt", (ModelNode*)lln0, 0, false);
    /* Name: visible string (read). */
    CDC_VSS_create("Name", (ModelNode*)lln0, 0);

    /* seed values — ModelNode_getChild takes the DO then the DA name. */
    ModelNode* health = ModelNode_getChild((ModelNode*)lln0, "Health");
    if (health) {
        DataAttribute* stVal = (DataAttribute*)ModelNode_getChild(health, "stVal");
        if (stVal) DataAttribute_setValue(stVal, MmsValue_newInteger(7));
    }
    ModelNode* meas = ModelNode_getChild((ModelNode*)lln0, "Meas");
    if (meas) {
        DataAttribute* mag = (DataAttribute*)ModelNode_getChild(meas, "instMag.f");
        if (mag) DataAttribute_setValue(mag, MmsValue_newFloat(42.5f));
    }
    ModelNode* setpt = ModelNode_getChild((ModelNode*)lln0, "SetPt");
    if (setpt) {
        DataAttribute* sm = (DataAttribute*)ModelNode_getChild(setpt, "setMag.f");
        if (sm) DataAttribute_setValue(sm, MmsValue_newFloat(1.0f));
    }
    ModelNode* name = ModelNode_getChild((ModelNode*)lln0, "Name");
    if (name) {
        DataAttribute* nm = (DataAttribute*)ModelNode_getChild(name, "val");
        if (nm) DataAttribute_setValue(nm, MmsValue_newVisibleString("TetherIED"));
    }

    /* RCB surface: a data set over Meas.instMag.f + Health.stVal and
       an unbuffered report control block (URCB, FC RP).  Clients
       enable it via SetRCBValues; data changes then trigger dchg
       InformationReports. */
    DataSet* ds = DataSet_create("measds", lln0);
    DataSetEntry_create(ds, "LLN0$MX$Meas$instMag$f", -1, NULL);
    DataSetEntry_create(ds, "LLN0$SP$SetPt$setMag$f", -1, NULL);
    ReportControlBlock_create("measurcb", lln0, "rptMeas", false,
                              "measds", 1,
                              TRG_OPT_DATA_CHANGED | TRG_OPT_GI,
                              RPT_OPT_SEQ_NUM | RPT_OPT_REASON_FOR_INCLUSION |
                              RPT_OPT_DATA_SET |
                              RPT_OPT_TIME_STAMP,
                              0, 0);
    /* buffered twin: same dataset, BR reference, 50 ms bufTm and an
       200 ms integrity period (TRG_OPT_INTEGRITY). */
    ReportControlBlock_create("bufrcb", lln0, "rptBuf", true,
                              "measds", 1,
                              TRG_OPT_DATA_CHANGED | TRG_OPT_GI |
                              TRG_OPT_INTEGRITY,
                              RPT_OPT_SEQ_NUM | RPT_OPT_REASON_FOR_INCLUSION |
                              RPT_OPT_DATA_SET |
                              RPT_OPT_TIME_STAMP,
                              50, 200);

    IedServer ied = IedServer_create(model);
    IedServer_setLocalIpAddress(ied, "127.0.0.1");
    IedServer_start(ied, port);

    printf("iec61850_ied_fake on %d\n", port);
    fflush(stdout);

    signal(SIGINT, onSig);
    signal(SIGTERM, onSig);
    while (!stopFlag)
        Thread_sleep(200);

    IedServer_stop(ied);
    IedServer_destroy(ied);
    return 0;
}
