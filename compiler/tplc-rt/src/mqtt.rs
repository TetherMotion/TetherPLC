//! tplc-rt builtin FB: mqtt.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Mqtt — MQTT client over tplc-mqtt (libpaho-mqtt3c).
// ---------------------------------------------------------------------------

// FB_Mqtt member offsets (see builtin_fb_layout("FB_Mqtt")).
// Strings are len-prefixed: HOST u8len@8+data@9 cap63, TOPIC
// u8len@84+data@85 cap127, PAYLOAD u8len@212+data@213 cap255,
// RXTOPIC u8len@476+data@477 cap127, RXPAYLOAD u8len@604+data@605
// cap255, RXRETAIN@872, STATE@876.
pub(crate) const MQ_OP: usize = 0;
pub(crate) const MQ_HANDLE: usize = 4;
pub(crate) const MQ_HOST_LEN: usize = 8;
pub(crate) const MQ_HOST: usize = 9;
pub(crate) const MQ_PORT: usize = 72;
pub(crate) const MQ_CLEAN: usize = 76;
pub(crate) const MQ_KEEPALIVE: usize = 80;
pub(crate) const MQ_TOPIC_LEN: usize = 84;
pub(crate) const MQ_TOPIC: usize = 85;
pub(crate) const MQ_PAYLOAD_LEN: usize = 212;
pub(crate) const MQ_PAYLOAD: usize = 213;
pub(crate) const MQ_QOS: usize = 468;
pub(crate) const MQ_FLG_RETAIN: usize = 472;
pub(crate) const MQ_RXTOPIC_LEN: usize = 476;
pub(crate) const MQ_RXTOPIC: usize = 477;
pub(crate) const MQ_RXPAYLOAD_LEN: usize = 604;
pub(crate) const MQ_RXPAYLOAD: usize = 605;
pub(crate) const MQ_RXQOS: usize = 860;
pub(crate) const MQ_RXMSGID: usize = 864;
pub(crate) const MQ_RXPLEN: usize = 868;
pub(crate) const MQ_RXRETAIN: usize = 872;
pub(crate) const MQ_STATE: usize = 876;
pub(crate) const MQ_LASTMSGID: usize = 880;
pub(crate) const MQ_LASTERR: usize = 884;
pub(crate) const MQ_STATUS: usize = 888;
pub(crate) const MQ_ERRID: usize = 892;
pub(crate) const MQ_CONNECTED: usize = 896;
// Will members (appended): WILLTOPIC u8len@900+data@901 cap127,
// WILLPAYLOAD u8len@1028+data@1029 cap255, WILLQOS@1284,
// WILLRETAIN@1288.
pub(crate) const MQ_WILLTOPIC_LEN: usize = 900;
pub(crate) const MQ_WILLTOPIC: usize = 901;
pub(crate) const MQ_WILLPAYLOAD_LEN: usize = 1028;
pub(crate) const MQ_WILLPAYLOAD: usize = 1029;
pub(crate) const MQ_WILLQOS: usize = 1284;
pub(crate) const MQ_WILLRETAIN: usize = 1288;
// TLS members (appended): USETLS@1292, TLSINSECURE@1296,
// TLSCAFILE u8len@1300+data@1301 cap127, TLSCERTFILE u8len@1428+
// data@1429 cap127, TLSKEYFILE u8len@1556+data@1557 cap127.
pub(crate) const MQ_USETLS: usize = 1292;
pub(crate) const MQ_TLSINSECURE: usize = 1296;
pub(crate) const MQ_TLSCA_LEN: usize = 1300;
pub(crate) const MQ_TLSCA: usize = 1301;
pub(crate) const MQ_TLSCERT_LEN: usize = 1428;
pub(crate) const MQ_TLSCERT: usize = 1429;
pub(crate) const MQ_TLSKEY_LEN: usize = 1556;
pub(crate) const MQ_TLSKEY: usize = 1557;
// LASTMSGSINCE (appended): udint@1684.
pub(crate) const MQ_LASTMSGSINCE: usize = 1684;
// Backoff members (appended): BACKOFFMIN@1688, BACKOFFMAX@1692,
// BACKOFFEN@1696.
pub(crate) const MQ_BACKOFFMIN: usize = 1688;
pub(crate) const MQ_BACKOFFMAX: usize = 1692;
pub(crate) const MQ_BACKOFFEN: usize = 1696;
// MQTT5 members (appended): MQTTVER@1700, M5SESSIONEXP@1704,
// M5RECVMAX@1708, M5TOPICALIAS@1712, M5REQRESP@1716, M5REQPROB@1720.
pub(crate) const MQ_MQTTVER: usize = 1700;
pub(crate) const MQ_M5SESSIONEXP: usize = 1704;
pub(crate) const MQ_M5RECVMAX: usize = 1708;
pub(crate) const MQ_M5TOPICALIAS: usize = 1712;
pub(crate) const MQ_M5REQRESP: usize = 1716;
pub(crate) const MQ_M5REQPROB: usize = 1720;
// MQTT5 publish-prop members (appended): M5PUBFMT@1724,
// M5PUBEXPIRY@1728, M5PUBCTYPE Str[127]@1732, M5PUBRTOPIC@1860,
// M5PUBCORR@1988, M5PUBCORR_LEN@2116, M5PUBUPROPS Str[255]@2120.
pub(crate) const MQ_M5PUBFMT: usize = 1724;
pub(crate) const MQ_M5PUBEXPIRY: usize = 1728;
pub(crate) const MQ_M5PUBCTYPE_LEN: usize = 1732;
pub(crate) const MQ_M5PUBCTYPE: usize = 1733;
pub(crate) const MQ_M5PUBRTOPIC_LEN: usize = 1860;
pub(crate) const MQ_M5PUBRTOPIC: usize = 1861;
pub(crate) const MQ_M5PUBCORR_LEN: usize = 1988;
pub(crate) const MQ_M5PUBCORR: usize = 1989;
#[allow(dead_code)] // reserved ABI slot
pub(crate) const MQ_M5PUBCORRDLEN: usize = 2116;
pub(crate) const MQ_M5PUBUPROPS_LEN: usize = 2120;
pub(crate) const MQ_M5PUBUPROPS: usize = 2121;
// MQTT5 subscribe-prop members (appended): M5SUBNOLOCAL@2376,
// M5SUBRAP@2380, M5SUBRH@2384, M5SUBSID@2388, M5SUBUPROPS Str[255]@2392.
pub(crate) const MQ_M5SUBNOLOCAL: usize = 2376;
pub(crate) const MQ_M5SUBRAP: usize = 2380;
pub(crate) const MQ_M5SUBRH: usize = 2384;
pub(crate) const MQ_M5SUBSID: usize = 2388;
pub(crate) const MQ_M5SUBUPROPS_LEN: usize = 2392;
pub(crate) const MQ_M5SUBUPROPS: usize = 2393;
// MQTT5 receive-prop members (appended): M5RXFMT@2648,
// M5RXEXPIRY@2652, M5RXSUBID@2656, M5RXCTYPE Str[63]@2660,
// M5RXRTOPIC Str[127]@2724, M5RXUPROPS Str[255]@2852,
// M5RXCORR_LEN@3108, M5RXCORR Str[127]@3112.
pub(crate) const MQ_M5RXFMT: usize = 2648;
pub(crate) const MQ_M5RXEXPIRY: usize = 2652;
pub(crate) const MQ_M5RXSUBID: usize = 2656;
pub(crate) const MQ_M5RXCTYPE_LEN: usize = 2660;
pub(crate) const MQ_M5RXCTYPE: usize = 2661;
pub(crate) const MQ_M5RXRTOPIC_LEN: usize = 2724;
pub(crate) const MQ_M5RXRTOPIC: usize = 2725;
pub(crate) const MQ_M5RXUPROPS_LEN: usize = 2852;
pub(crate) const MQ_M5RXUPROPS: usize = 2853;
pub(crate) const MQ_M5RXCORRDLEN: usize = 3108;
pub(crate) const MQ_M5RXCORR_LEN: usize = 3112;
pub(crate) const MQ_M5RXCORR: usize = 3113;
// Connect extras (appended): CONNECTMS@3240,
// USERNAME Str[63] len@3244 data@3245, PASSWORD Str[127]
// len@3308 data@3309, CLIENTID Str[63] len@3436 data@3437.
pub(crate) const MQ_CONNECTMS: usize = 3240;
pub(crate) const MQ_USERNAME_LEN: usize = 3244;
pub(crate) const MQ_USERNAME: usize = 3245;
pub(crate) const MQ_PASSWORD_LEN: usize = 3308;
pub(crate) const MQ_PASSWORD: usize = 3309;
pub(crate) const MQ_CLIENTID_LEN: usize = 3436;
pub(crate) const MQ_CLIENTID: usize = 3437;
// TRANSPORT udint appended @3500 -> TplcMqttConnectCfg.transport@124
pub(crate) const MQ_TRANSPORT: usize = 3500;

// tplc-mqtt service vtable offsets (TplcMqttService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const MQ_SVC_CONNECT: usize = 16;
pub(crate) const MQ_SVC_DISCONNECT: usize = 24;
pub(crate) const MQ_SVC_STATUS: usize = 32;
pub(crate) const MQ_SVC_PUBLISH: usize = 40;
pub(crate) const MQ_SVC_SUBSCRIBE: usize = 48;
pub(crate) const MQ_SVC_UNSUBSCRIBE: usize = 56;
pub(crate) const MQ_SVC_RECEIVE: usize = 64;
pub(crate) const MQ_SVC_IS_CONNECTED: usize = 72;
pub(crate) const MQ_SVC_ABORT: usize = 80;
pub(crate) const MQ_SVC_BACKOFF: usize = 88;
pub(crate) const MQ_SVC_PUBLISH5: usize = 96;
pub(crate) const MQ_SVC_SUBSCRIBE5: usize = 104;
pub(crate) const MQ_SVC_RECEIVE5: usize = 112;

/// service lookup for "tplc-mqtt" (9 chars)
pub(crate) unsafe fn mqtt_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-mqtt".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn mqfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FMqttConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FMqttDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FMqttStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMqttPublish = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, u32, u32, u32) -> i32;
pub(crate) type FMqttSubscribe = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
pub(crate) type FMqttReceive = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMqttReceive5 = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMqttIsConnected = unsafe extern "C" fn(*const u8, u32) -> i32;

/// `__tplc_rt_FB_Mqtt` — synchronous op dispatch over the tplc-mqtt
/// MQTT client. OP: 1=connect (HOST/PORT/CLEAN/KEEPALIVE -> HANDLE),
/// 2=disconnect, 3=status (-> STATE/LASTMSGID/LASTERR), 4=publish
/// (TOPIC/PAYLOAD/QOS/RETAIN), 5=subscribe (TOPIC/QOS),
/// 6=unsubscribe (TOPIC), 7=receive (-> RXTOPIC/RXPAYLOAD/RXQOS/
/// RXMSGID/RXPLEN or NOMSG), 8=isConnected (-> CONNECTED). STATUS
/// carries the service return code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Mqtt(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MQ_OP));
    wi32(this.add(MQ_STATUS), 0);
    wu32(this.add(MQ_ERRID), 0);
    let svc = mqtt_service(ctx);
    if svc.is_null() {
        wi32(this.add(MQ_STATUS), -2);
        wu32(this.add(MQ_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(MQ_STATUS), rc);
        wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, MQ_HOST_LEN, MQ_HOST, 63, buf.as_mut_ptr());
            // TplcMqttConnectCfg (128B): clean@0 i32, keepalive@4 u32,
            // version@8 u32, timeout@12 u32, username@16 ptr,
            // password@24 ptr, client_id@32 ptr, will_topic@40 ptr,
            // will_payload@48 ptr, will_payload_len@56 u32,
            // will_qos@60 u32, will_retain@64 u32, use_tls@68 u32,
            // tls_insecure@72 u32, tls_ca@80 ptr, tls_cert@88 ptr,
            // tls_key@96 ptr, m5 props@104..120, transport@124 u32
            let mut cfg = [0u8; 128];
            let cfgp = cfg.as_mut_ptr();
            let wi = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), cfgp.add(off), 4);
            };
            wi(0, ru32(this.add(MQ_CLEAN)) as i32);
            wi(4, ru32(this.add(MQ_KEEPALIVE)) as i32);
            let mver = ru32(this.add(MQ_MQTTVER));
            wi(8, if mver == 5 { 5 } else { 3 });
            let cms = ru32(this.add(MQ_CONNECTMS));
            wi(12, if cms > 0 { cms as i32 } else { 5000 });
            wi(124, ru32(this.add(MQ_TRANSPORT)) as i32);
            // username/password/client_id — stack buffers stay alive
            // through the synchronous connect call
            let mut ubuf = [0u8; 64];
            let mut pbuf2 = [0u8; 128];
            let mut cbuf = [0u8; 64];
            if *this.add(MQ_USERNAME_LEN) > 0 {
                uacstr(this, MQ_USERNAME_LEN, MQ_USERNAME, 63,
                       ubuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (ubuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(16), 8);
            }
            if *this.add(MQ_PASSWORD_LEN) > 0 {
                uacstr(this, MQ_PASSWORD_LEN, MQ_PASSWORD, 127,
                       pbuf2.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (pbuf2.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(24), 8);
            }
            if *this.add(MQ_CLIENTID_LEN) > 0 {
                uacstr(this, MQ_CLIENTID_LEN, MQ_CLIENTID, 63,
                       cbuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (cbuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(32), 8);
            }
            let mut wbuf = [0u8; 128];
            let mut pbuf = [0u8; 256];
            if *this.add(MQ_WILLTOPIC_LEN) > 0 {
                uacstr(this, MQ_WILLTOPIC_LEN, MQ_WILLTOPIC, 127,
                       wbuf.as_mut_ptr());
                let plen = uacstr(this, MQ_WILLPAYLOAD_LEN,
                                  MQ_WILLPAYLOAD, 255, pbuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (wbuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(40), 8);
                core::ptr::copy_nonoverlapping(
                    (pbuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(48), 8);
                wi(56, plen as i32);
                wi(60, ru32(this.add(MQ_WILLQOS)) as i32);
                wi(64, ru32(this.add(MQ_WILLRETAIN)) as i32);
            }
            // TLS: file paths copied into stack buffers that stay
            // alive through the synchronous connect call.
            wi(68, ru32(this.add(MQ_USETLS)) as i32);
            wi(72, ru32(this.add(MQ_TLSINSECURE)) as i32);
            let mut cabuf = [0u8; 128];
            let mut certbuf = [0u8; 128];
            let mut keybuf = [0u8; 128];
            if *this.add(MQ_TLSCA_LEN) > 0 {
                uacstr(this, MQ_TLSCA_LEN, MQ_TLSCA, 127,
                       cabuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (cabuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(80), 8);
            }
            if *this.add(MQ_TLSCERT_LEN) > 0 {
                uacstr(this, MQ_TLSCERT_LEN, MQ_TLSCERT, 127,
                       certbuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (certbuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(88), 8);
            }
            if *this.add(MQ_TLSKEY_LEN) > 0 {
                uacstr(this, MQ_TLSKEY_LEN, MQ_TLSKEY, 127,
                       keybuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (keybuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    cfgp.add(96), 8);
            }
            // MQTT5 connect properties (cfg offsets 104..120)
            wi(104, ru32(this.add(MQ_M5SESSIONEXP)) as i32);
            wi(108, ru32(this.add(MQ_M5RECVMAX)) as i32);
            wi(112, ru32(this.add(MQ_M5TOPICALIAS)) as i32);
            wi(116, ru32(this.add(MQ_M5REQRESP)) as i32);
            wi(120, ru32(this.add(MQ_M5REQPROB)) as i32);
            let f: FMqttConnect = core::mem::transmute(mqfn(svc, MQ_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(MQ_PORT)),
                       cfg.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(MQ_HANDLE), h);
            }
        }
        2 => {
            let f: FMqttDisconnect = core::mem::transmute(mqfn(svc, MQ_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE))));
        }
        3 => {
            // TplcMqttStatus (48B): state@0 conn_flags@4 last_msg_id@8
            // last_qos@12 last_error@16
            let mut st = [0u8; 48];
            let f: FMqttStatus = core::mem::transmute(mqfn(svc, MQ_SVC_STATUS));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), st.as_mut_ptr());
            wi32(this.add(MQ_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MQ_STATE), rd(0));
                wu32(this.add(MQ_LASTMSGID), rd(8) as u32);
                wi32(this.add(MQ_LASTERR), rd(16));
                wu32(this.add(MQ_LASTMSGSINCE), rd(20) as u32);
            } else {
                wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let n = (*this.add(MQ_PAYLOAD_LEN) as usize).min(255);
            let f: FMqttPublish = core::mem::transmute(mqfn(svc, MQ_SVC_PUBLISH));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)),
                       tbuf.as_ptr(), this.add(MQ_PAYLOAD), n as u32,
                       ru32(this.add(MQ_QOS)), ru32(this.add(MQ_FLG_RETAIN)));
            set_rc(rc);
        }
        5 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let f: FMqttSubscribe = core::mem::transmute(mqfn(svc, MQ_SVC_SUBSCRIBE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)),
                       tbuf.as_ptr(), ru32(this.add(MQ_QOS)));
            set_rc(rc);
        }
        6 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let f: FMqttSubscribe = core::mem::transmute(mqfn(svc, MQ_SVC_UNSUBSCRIBE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), tbuf.as_ptr(), 0);
            set_rc(rc);
        }
        7 => {
            // TplcMqttRx: caps@0/4, lens@8/12, qos@16, message_id@20,
            // retained@24, topic ptr@32, payload ptr@40
            let mut rx = [0u8; 48];
            let mut wu = |off: usize, v: u32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), rx.as_mut_ptr().add(off), 4);
            };
            wu(0, 127);
            wu(4, 255);
            // topic ptr @32, payload ptr @40 (usize on 64-bit)
            let tp = this.add(MQ_RXTOPIC) as usize;
            let pp = this.add(MQ_RXPAYLOAD) as usize;
            core::ptr::copy_nonoverlapping(
                (tp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(32), 8);
            core::ptr::copy_nonoverlapping(
                (pp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(40), 8);
            let f: FMqttReceive = core::mem::transmute(mqfn(svc, MQ_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), rx.as_mut_ptr());
            wi32(this.add(MQ_STATUS), rc);
            wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rx.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let tl = rd(8).min(127) as usize;
                *this.add(MQ_RXTOPIC_LEN) = tl as u8;
                *this.add(MQ_RXTOPIC + tl) = 0;
                let pl = rd(12).min(255) as usize;
                *this.add(MQ_RXPAYLOAD_LEN) = pl as u8;
                wu32(this.add(MQ_RXQOS), rd(16));
                wu32(this.add(MQ_RXMSGID), rd(20));
                wu32(this.add(MQ_RXRETAIN), rd(24));
                wu32(this.add(MQ_RXPLEN), rd(12));
            }
        }
        8 => {
            let f: FMqttIsConnected = core::mem::transmute(mqfn(svc, MQ_SVC_IS_CONNECTED));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)));
            wi32(this.add(MQ_STATUS), rc);
            wu32(this.add(MQ_CONNECTED), if rc == 1 { 1 } else { 0 });
        }
        9 => {
            // abort: drop TCP without DISCONNECT -> broker fires will
            let f: FMqttDisconnect = core::mem::transmute(mqfn(svc, MQ_SVC_ABORT));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE))));
        }
        10 => {
            // exponential backoff: BACKOFFMIN/MAX/EN -> watchdog
            type FMqttBackoff = unsafe extern "C" fn(
                *const u8, u32, u32, u32, u32) -> i32;
            let f: FMqttBackoff =
                core::mem::transmute(mqfn(svc, MQ_SVC_BACKOFF));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE)),
                     ru32(this.add(MQ_BACKOFFMIN)),
                     ru32(this.add(MQ_BACKOFFMAX)),
                     ru32(this.add(MQ_BACKOFFEN))));
        }
        11 => {
            // MQTT5 publish with properties -> TplcMqtt5PubProps
            // (48B): fmt@0 expiry@4 ctype@8 rtopic@16 corr@24
            // corrlen@32 uprops@40
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let n = (*this.add(MQ_PAYLOAD_LEN) as usize).min(255);
            let mut pr = [0u8; 48];
            let pp = pr.as_mut_ptr();
            let pw = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), pp.add(off), 4);
            };
            let pptr = |off: usize, p: *const u8| {
                core::ptr::copy_nonoverlapping(
                    (p as usize).to_le_bytes().as_ptr(), pp.add(off), 8);
            };
            pw(0, ru32(this.add(MQ_M5PUBFMT)) as i32);
            pw(4, ru32(this.add(MQ_M5PUBEXPIRY)) as i32);
            let mut cbuf = [0u8; 128];
            let mut rbuf = [0u8; 128];
            let mut xbuf = [0u8; 128];
            let mut ubuf = [0u8; 256];
            if *this.add(MQ_M5PUBCTYPE_LEN) > 0 {
                uacstr(this, MQ_M5PUBCTYPE_LEN, MQ_M5PUBCTYPE, 127,
                       cbuf.as_mut_ptr());
                pptr(8, cbuf.as_ptr());
            }
            if *this.add(MQ_M5PUBRTOPIC_LEN) > 0 {
                uacstr(this, MQ_M5PUBRTOPIC_LEN, MQ_M5PUBRTOPIC, 127,
                       rbuf.as_mut_ptr());
                pptr(16, rbuf.as_ptr());
            }
            if *this.add(MQ_M5PUBCORR_LEN) > 0 {
                let cn = uacstr(this, MQ_M5PUBCORR_LEN, MQ_M5PUBCORR,
                                127, xbuf.as_mut_ptr());
                pptr(24, xbuf.as_ptr());
                pw(32, cn as i32);
            }
            if *this.add(MQ_M5PUBUPROPS_LEN) > 0 {
                uacstr(this, MQ_M5PUBUPROPS_LEN, MQ_M5PUBUPROPS, 255,
                       ubuf.as_mut_ptr());
                pptr(40, ubuf.as_ptr());
            }
            type FMqttPublish5 = unsafe extern "C" fn(
                *const u8, u32, *const u8, *const u8, u32, u32, u32,
                *const u8) -> i32;
            let f: FMqttPublish5 =
                core::mem::transmute(mqfn(svc, MQ_SVC_PUBLISH5));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE)),
                     tbuf.as_ptr(), this.add(MQ_PAYLOAD), n as u32,
                     ru32(this.add(MQ_QOS)), ru32(this.add(MQ_FLG_RETAIN)),
                     pr.as_ptr()));
        }
        12 => {
            // MQTT5 subscribe with properties -> TplcMqtt5SubProps
            // (24B): nolocal@0 rap@4 rh@8 sid@12 uprops@16
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let mut pr = [0u8; 24];
            let pp = pr.as_mut_ptr();
            let pw = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), pp.add(off), 4);
            };
            let mut ubuf = [0u8; 256];
            pw(0, ru32(this.add(MQ_M5SUBNOLOCAL)) as i32);
            pw(4, ru32(this.add(MQ_M5SUBRAP)) as i32);
            pw(8, ru32(this.add(MQ_M5SUBRH)) as i32);
            pw(12, ru32(this.add(MQ_M5SUBSID)) as i32);
            if *this.add(MQ_M5SUBUPROPS_LEN) > 0 {
                uacstr(this, MQ_M5SUBUPROPS_LEN, MQ_M5SUBUPROPS, 255,
                       ubuf.as_mut_ptr());
                core::ptr::copy_nonoverlapping(
                    (ubuf.as_ptr() as usize).to_le_bytes().as_ptr(),
                    pp.add(16), 8);
            }
            type FMqttSubscribe5 = unsafe extern "C" fn(
                *const u8, u32, *const u8, u32, *const u8) -> i32;
            let f: FMqttSubscribe5 =
                core::mem::transmute(mqfn(svc, MQ_SVC_SUBSCRIBE5));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE)),
                     tbuf.as_ptr(), ru32(this.add(MQ_QOS)),
                     pr.as_ptr()));
        }
        13 => {
            // MQTT5 receive with properties -> TplcMqttRx5 (648B):
            // base TplcMqttRx @0..47, then fmt@48 expiry@52 subid@56
            // ctype@64[64] rtopic@128[128] uprops@256[256]
            // corr_len@512 corr@516[128]
            let mut rx = [0u8; 648];
            let mut wu = |off: usize, v: u32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), rx.as_mut_ptr().add(off), 4);
            };
            wu(0, 127);
            wu(4, 255);
            let tp = this.add(MQ_RXTOPIC) as usize;
            let pp = this.add(MQ_RXPAYLOAD) as usize;
            core::ptr::copy_nonoverlapping(
                (tp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(32), 8);
            core::ptr::copy_nonoverlapping(
                (pp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(40), 8);
            let f: FMqttReceive5 = core::mem::transmute(
                mqfn(svc, MQ_SVC_RECEIVE5));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), rx.as_mut_ptr());
            wi32(this.add(MQ_STATUS), rc);
            wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rx.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let tl = rd(8).min(127) as usize;
                *this.add(MQ_RXTOPIC_LEN) = tl as u8;
                *this.add(MQ_RXTOPIC + tl) = 0;
                let pl = rd(12).min(255) as usize;
                *this.add(MQ_RXPAYLOAD_LEN) = pl as u8;
                wu32(this.add(MQ_RXQOS), rd(16));
                wu32(this.add(MQ_RXMSGID), rd(20));
                wu32(this.add(MQ_RXRETAIN), rd(24));
                wu32(this.add(MQ_RXPLEN), rd(12));
                wu32(this.add(MQ_M5RXFMT), rd(48));
                wu32(this.add(MQ_M5RXEXPIRY), rd(52));
                wu32(this.add(MQ_M5RXSUBID), rd(56));
                // NUL-terminated prop strings -> Str members
                // (len byte + data); caps 63/127/255
                let cstr = |dst_len: usize, dst: usize, src: usize,
                            cap: usize| {
                    let mut n = 0usize;
                    while n < cap && rx[src + n] != 0 { n += 1; }
                    *this.add(dst_len) = n as u8;
                    core::ptr::copy_nonoverlapping(
                        rx.as_ptr().add(src), this.add(dst), n);
                    *this.add(dst + n) = 0;
                };
                cstr(MQ_M5RXCTYPE_LEN, MQ_M5RXCTYPE, 64, 63);
                cstr(MQ_M5RXRTOPIC_LEN, MQ_M5RXRTOPIC, 128, 127);
                cstr(MQ_M5RXUPROPS_LEN, MQ_M5RXUPROPS, 256, 255);
                // correlation data @516, len @512 (binary)
                let cn = rd(512).min(127) as usize;
                wu32(this.add(MQ_M5RXCORRDLEN), cn as u32);
                *this.add(MQ_M5RXCORR_LEN) = cn as u8;
                core::ptr::copy_nonoverlapping(
                    rx.as_ptr().add(516), this.add(MQ_M5RXCORR), cn);
                *this.add(MQ_M5RXCORR + cn) = 0;
            }
        }
        _ => {
            wi32(this.add(MQ_STATUS), -1);
            wu32(this.add(MQ_ERRID), 1);
        }
    }
    0
}
