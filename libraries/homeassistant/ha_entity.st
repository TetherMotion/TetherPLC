// ha_entity.st — FB_HaEntity: a single Home Assistant entity as one
// easy-to-wire FBD block.  Wraps FB_HaBridge with exactly one channel
// so EVERY option is an input pin — no member-array configuration.
// For many entities on one MQTT connection use FB_HaBridge directly
// (one FB_Mqtt connection per entity here).
//
// Entity kinds (Kind pin):
//   0 sensor        PLC->HA   Value published to state topic
//   1 binary_sensor PLC->HA   BVal published ON/OFF
//   2 switch        both      BVal published; HA cmd -> Cmd/RxVal,
//                             AutoWr latches BVal
//   3 number        both      Value published; HA cmd -> RxVal
//                             (clamped MinV..MaxV), AutoWr latches
//   4 button        HA->PLC   cmd topic -> Event pulse
//   5 text          both      Txt published; cmd -> RxTxt,
//                             AutoWr latches Txt
//   6 event         HA->PLC   ObjId subscribed as topic; any msg ->
//                             Event pulse + RxTxt
//
// Value pins are "desired value" inputs: copied into the entity only
// when they change, so HA AutoWr commands are not overwritten.
// ForceWr forces a pin->entity copy (breaks an AutoWr latch).
// OutValue/OutBVal/OutTxt always show the entity's actual state.

FUNCTION_BLOCK FB_HaEntity
VAR_INPUT
    // ---- link / bridge ----
    Execute : BOOL;
    CycleTime : LREAL;
    Dev : STRING[31];
    Host : STRING[63];
    Port : UDINT;
    User : STRING[63];
    Pass : STRING[127];
    ClientId : STRING[63];
    ConnectMs : UDINT;
    MqttVer : UDINT;
    Transport : UDINT;       // 0 auto,1 tcp,2 ssl,3 ws,4 wss
    KeepAlive : UDINT;
    Clean : BOOL;
    UseTls : BOOL;
    TlsInsecure : BOOL;
    TlsCaFile : STRING[63];
    TlsCertFile : STRING[63];
    TlsKeyFile : STRING[63];
    WillTopic : STRING[127];
    WillMsg : STRING[127];
    WillQoS : UDINT;
    WillRetain : BOOL;
    BackoffMin : UDINT;
    BackoffMax : UDINT;
    HbS : UDINT;
    // ---- entity ----
    En : BOOL;
    Kind : UDINT;            // 0..6 per header
    ObjId : STRING[31];      // entity object id (HA unique_id part)
    Name : STRING[31];
    Unit : STRING[15];
    DevCls : STRING[31];     // HA device_class (temperature, ...)
    MinV : LREAL;
    MaxV : LREAL;
    StepV : LREAL;
    AutoWr : BOOL;           // HA cmd latches into published value
    ForceWr : BOOL;          // TRUE this scan = copy pins unconditionally
    Value : LREAL;           // desired analog value (kinds 0,3)
    BVal : BOOL;             // desired bool state (kinds 1,2)
    Txt : STRING[63];        // desired text (kind 5)
END_VAR
VAR_OUTPUT
    // ---- bridge status ----
    Connected : BOOL;
    Err : BOOL;
    ErrId : UDINT;
    State : DINT;
    TxCnt : UDINT;
    RxCnt : UDINT;
    // ---- entity state (actual, incl. AutoWr'd commands) ----
    OutValue : LREAL;
    OutBVal : BOOL;
    OutTxt : STRING[63];
    // ---- received commands ----
    HasMsg : BOOL;
    MsgCnt : UDINT;
    Cmd : DINT;              // switch: 1 ON 2 OFF 3 TOGGLE
    RxVal : LREAL;
    RxTxt : STRING[63];
    Event : BOOL;            // button/event pulse
END_VAR
VAR
    br : FB_HaBridge;
    pv : LREAL;
    pb : BOOL;
    pt : STRING[63];
END_VAR
    // ---- desired-value pins -> channel (change-detected) ----
    IF ForceWr OR Value <> pv THEN
        br.CHAN[1].Value := Value;
        pv := Value;
    END_IF;
    IF ForceWr OR BVal <> pb THEN
        br.CHAN[1].BVal := BVal;
        pb := BVal;
    END_IF;
    IF ForceWr OR Txt <> pt THEN
        br.CHAN[1].Txt := Txt;
        pt := Txt;
    END_IF;
    // ---- static entity config ----
    br.CHAN[1].En := En;
    br.CHAN[1].Kind := Kind;
    br.CHAN[1].ObjId := ObjId;
    br.CHAN[1].Name := Name;
    br.CHAN[1].Unit := Unit;
    br.CHAN[1].DevCls := DevCls;
    br.CHAN[1].MinV := MinV;
    br.CHAN[1].MaxV := MaxV;
    br.CHAN[1].StepV := StepV;
    br.CHAN[1].AutoWr := AutoWr;
    // ---- run the bridge ----
    br.Execute := Execute;
    br.CycleTime := CycleTime;
    br.Dev := Dev;
    br.Host := Host;
    br.Port := Port;
    br.User := User;
    br.Pass := Pass;
    br.ClientId := ClientId;
    br.ConnectMs := ConnectMs;
    br.MqttVer := MqttVer;
    br.Transport := Transport;
    br.KeepAlive := KeepAlive;
    br.Clean := Clean;
    br.UseTls := UseTls;
    br.TlsInsecure := TlsInsecure;
    br.TlsCaFile := TlsCaFile;
    br.TlsCertFile := TlsCertFile;
    br.TlsKeyFile := TlsKeyFile;
    br.WillTopic := WillTopic;
    br.WillMsg := WillMsg;
    br.WillQoS := WillQoS;
    br.WillRetain := WillRetain;
    br.BackoffMin := BackoffMin;
    br.BackoffMax := BackoffMax;
    br.HbS := HbS;
    br();
    // ---- outputs ----
    Connected := br.Connected;
    Err := br.Err;
    ErrId := br.ErrId;
    State := br.State;
    TxCnt := br.TxCnt;
    RxCnt := br.RxCnt;
    OutValue := br.CHAN[1].Value;
    OutBVal := br.CHAN[1].BVal;
    OutTxt := br.CHAN[1].Txt;
    HasMsg := br.CHAN[1].HasMsg;
    MsgCnt := br.CHAN[1].MsgCnt;
    Cmd := br.CHAN[1].Cmd;
    RxVal := br.CHAN[1].RxVal;
    RxTxt := br.CHAN[1].RxTxt;
    Event := br.CHAN[1].Event;
END_FUNCTION_BLOCK
