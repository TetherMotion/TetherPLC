// tc2_aes70_case.st — OCA/AES70 client e2e over the python device
// fake.  Connect, keepalive, a method call -> echoed response,
// disconnect.
PROGRAM tc2_aes70_case
VAR
    con : FB_Aes70Connect;
    dis : FB_Aes70Disconnect;
    ka  : FB_Aes70Keepalive;
    cal : FB_Aes70Call;
    rx  : FB_Aes70Receive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.Host := '127.0.0.1';
        con.Port := 65000;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: session up
            ka.Execute := TRUE;
            ka.ConnectionHdl := hdl;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF ka.Done THEN
            mask := mask + 2;         // bit1: keepalive sent
            ka.Execute := FALSE;
            cal.Execute := TRUE;
            cal.ConnectionHdl := hdl;
            cal.TargetONo := 1000;    // an OCA object number
            cal.MethodID := 260;      // defLevel=1, methodIndex=4
            cal.Params := '{"gain":0.5}';
            st := 3;
        END_IF;
        IF ka.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF cal.Done THEN
            mask := mask + 4;         // bit2: call sent
            cal.Execute := FALSE;
            rx.Execute := TRUE;
            rx.ConnectionHdl := hdl;
            st := 4;
        END_IF;
        IF cal.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // poll for the device's echoed OcaResponse
    IF st = 4 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            mask := mask + 8;         // bit3: response landed
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 5;
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 40 THEN
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 5;
        END_IF;
    END_IF;

    IF st = 5 THEN
        IF dis.Done THEN
            mask := mask + 16;        // bit4: released
            st := 6;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); ka(); cal(); rx();
END_PROGRAM
