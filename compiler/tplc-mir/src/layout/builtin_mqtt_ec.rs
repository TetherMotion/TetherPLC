//! builtin_fb_layout arms — mqtt/ec domain.
use super::*;

        /* FB_Mqtt — tplc-mqtt service bridge (MQTT client via
        form.  Topics are bounded STRING[127], payloads STRING[255]. */
pub(super) fn arm_fb_mqtt(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "HOST", Type::Str { cap: 63 });
            push(&mut off, "PORT", udint());
            push(&mut off, "CLEAN", udint());
            push(&mut off, "KEEPALIVE", udint());
            push(&mut off, "TOPIC", Type::Str { cap: 127 });
            push(&mut off, "PAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "QOS", udint());
            push(&mut off, "FLG_RETAIN", udint());
            push(&mut off, "RXTOPIC", Type::Str { cap: 127 });
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXQOS", udint());
            push(&mut off, "RXMSGID", udint());
            push(&mut off, "RXPLEN", udint());
            push(&mut off, "RXRETAIN", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "LASTMSGID", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CONNECTED", udint());
            /* last-will + abort (appended): OP1 reads WILLTOPIC/
               WILLPAYLOAD/WILLQOS/WILLRETAIN; OP9 aborts the
               connection without DISCONNECT so the broker fires
               the will. */
            push(&mut off, "WILLTOPIC", Type::Str { cap: 127 });
            push(&mut off, "WILLPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "WILLQOS", udint());
            push(&mut off, "WILLRETAIN", udint());
            /* TLS (appended): OP1 reads USETLS/TLSINSECURE/
               TLSCAFILE/TLSCERTFILE/TLSKEYFILE into the connect
               cfg's ssl fields. */
            push(&mut off, "USETLS", udint());
            push(&mut off, "TLSINSECURE", udint());
            push(&mut off, "TLSCAFILE", Type::Str { cap: 127 });
            push(&mut off, "TLSCERTFILE", Type::Str { cap: 127 });
            push(&mut off, "TLSKEYFILE", Type::Str { cap: 127 });
            /* broker-message age (appended): OP3 copies
               TplcMqttStatus.last_msg_age_ms here. */
            push(&mut off, "LASTMSGSINCE", udint());
            /* backoff (appended): OP10 reads BACKOFFMIN/MAX/EN ->
               mqtt_backoff vtable op. */
            push(&mut off, "BACKOFFMIN", udint());
            push(&mut off, "BACKOFFMAX", udint());
            push(&mut off, "BACKOFFEN", udint());
            /* MQTT5 (appended): OP1 reads MQTTVER (3|5) + the five
               connect-property udints into the cfg tail. */
            push(&mut off, "MQTTVER", udint());
            push(&mut off, "M5SESSIONEXP", udint());
            push(&mut off, "M5RECVMAX", udint());
            push(&mut off, "M5TOPICALIAS", udint());
            push(&mut off, "M5REQRESP", udint());
            push(&mut off, "M5REQPROB", udint());
            /* MQTT5 publish properties (appended): OP11 reads these
               into TplcMqtt5PubProps for mqtt_publish5. */
            push(&mut off, "M5PUBFMT", udint());
            push(&mut off, "M5PUBEXPIRY", udint());
            push(&mut off, "M5PUBCTYPE", Type::Str { cap: 127 });
            push(&mut off, "M5PUBRTOPIC", Type::Str { cap: 127 });
            push(&mut off, "M5PUBCORR", Type::Str { cap: 127 });
            push(&mut off, "M5PUBCORR_LEN", udint());
            push(&mut off, "M5PUBUPROPS", Type::Str { cap: 255 });
            /* MQTT5 subscribe properties (appended): OP12 reads these
               into TplcMqtt5SubProps for mqtt_subscribe5. */
            push(&mut off, "M5SUBNOLOCAL", udint());
            push(&mut off, "M5SUBRAP", udint());
            push(&mut off, "M5SUBRH", udint());
            push(&mut off, "M5SUBSID", udint());
            push(&mut off, "M5SUBUPROPS", Type::Str { cap: 255 });
            /* MQTT5 receive properties (appended): OP13 reads these
               from TplcMqttRx5 for mqtt_receive5. */
            push(&mut off, "M5RXFMT", udint());
            push(&mut off, "M5RXEXPIRY", udint());
            push(&mut off, "M5RXSUBID", udint());
            push(&mut off, "M5RXCTYPE", Type::Str { cap: 63 });
            push(&mut off, "M5RXRTOPIC", Type::Str { cap: 127 });
            push(&mut off, "M5RXUPROPS", Type::Str { cap: 255 });
            push(&mut off, "M5RXCORR_LEN", udint());
            push(&mut off, "M5RXCORR", Type::Str { cap: 127 });
            /* connect extras (appended): OP1 reads CONNECTMS into the
               cfg timeout, USERNAME/PASSWORD/CLIENTID into the
               username/password/client_id ptrs. */
            push(&mut off, "CONNECTMS", udint());
            push(&mut off, "USERNAME", Type::Str { cap: 63 });
            push(&mut off, "PASSWORD", Type::Str { cap: 127 });
            push(&mut off, "CLIENTID", Type::Str { cap: 63 });
            /* transport selector (appended): OP1 reads TRANSPORT
               into TplcMqttConnectCfg.transport — 0=auto, 1=tcp,
               2=ssl, 3=ws, 4=wss (MQTT over WebSocket[+TLS]). */
            push(&mut off, "TRANSPORT", udint());
    off
}

        /* FB_EcSync — tplc-ecsync service bridge (EtherCAT DC sync
        service return code, ERRID its positive form. */
pub(super) fn arm_fb_ec_sync(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ENABLE", udint());
            push(&mut off, "CONNECTED", dint());
            push(&mut off, "SYNCEN", dint());
            push(&mut off, "PARAMSOK", dint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DIFFNS", Type::Real { bits: 64 });
            push(&mut off, "REFNS", Type::Int {
                signed: false,
                bits: 64,
            });
    off
}

        /* FB_EcDiagRead — tplc-ecdiag service bridge (EtherCAT CoE
        return code, ERRID its positive form. */
pub(super) fn arm_fb_ec_diag_read(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let arr32 = || Type::Array {
                elem: Box::new(udint()),
                lo: 0,
                hi: 15,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "SLAVE", udint());
            push(&mut off, "ACKVAL", udint());
            push(&mut off, "COUNT", udint());
            push(&mut off, "NEWEST", udint());
            push(&mut off, "ACKED", udint());
            push(&mut off, "NEWMSGS", udint());
            push(&mut off, "INFOFLAGS", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "MSGTYPE", arr32());
            push(&mut off, "ACKST", arr32());
            push(&mut off, "TSMS", arr32());
            push(&mut off, "CODE", arr32());
            push(&mut off, "CODETYPE", arr32());
            push(&mut off, "TEXTID", arr32());
            push(&mut off, "MODULE", arr32());
            push(&mut off, "CHANNEL", arr32());
            push(&mut off, "MSGTEXT", Type::Array {
                elem: Box::new(Type::Str { cap: 63 }),
                lo: 0,
                hi: 15,
            });
    off
}
