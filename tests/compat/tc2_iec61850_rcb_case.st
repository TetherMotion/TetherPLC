// tc2_iec61850_rcb_case.st — IEC 61850 RCB e2e over the fake IED:
// GetRCBValues on the URCB, SetRCBValues enable, write a dataset
// member, receive the dchg InformationReport via report poll, send
// GI for a full-dataset report, disable, disconnect.
// dchg reports carry only changed elements ("-" for the rest);
// GI reports carry every dataset element.
PROGRAM tc2_iec61850_rcb_case
VAR
    con : FB_Iec61850Connect;
    dis : FB_Iec61850Disconnect;
    rget : FB_Iec61850RCBGet;
    rset : FB_Iec61850RCBSet;
    poll : FB_Iec61850ReportPoll;
    wr  : FB_Iec61850Write;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
    rcbref : STRING[127];
    rep : STRING[255];
END_VAR
    IF st = 0 THEN
        mask := 0;
        rcbref := 'tetherIEDtetherTestLD/LLN0$$RP$$measurcb';
        con.Execute := TRUE;
        con.RemoteHost := '127.0.0.1';
        con.RemotePort := 11020;
        st := 1;
    ELSIF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;
            rget.Execute := TRUE;
            rget.ConnectionHdl := hdl;
            rget.RcbRef := rcbref;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 2 THEN
        IF rget.Done THEN
            rget.Execute := FALSE;
            // rptId 'rptMeas', confRev 1, unbuffered, dchg+gi trgOps
            IF LEN(rget.RptId) = 7 AND FIND(rget.RptId, 'rptMeas') = 1
               AND rget.ConfRev = 1 AND NOT rget.Buffered
               AND (rget.TrgOps AND 16#11) <> 0 THEN
                mask := mask + 2;
                rset.Execute := TRUE;
                rset.ConnectionHdl := hdl;
                rset.RcbRef := rcbref;
                rset.Flags := 1;      // rptEna
                st := 3;
            ELSE
                mask := 16#DEAD; st := 99;
            END_IF;
        END_IF;
        IF rget.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 3 THEN
        IF rset.Done THEN
            rset.Execute := FALSE;
            mask := mask + 4;
            wr.Execute := TRUE;
            wr.ConnectionHdl := hdl;
            wr.Domain := 'tetherIEDtetherTestLD';
            wr.Item := 'LLN0.SetPt.setMag.f#SP';
            wr.Value := '2.5';
            st := 4;
        END_IF;
        IF rset.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 4 THEN
        IF wr.Done THEN
            wr.Execute := FALSE;
            mask := mask + 8;
            waitN := 0;
            st := 5;
        END_IF;
        IF wr.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 5 THEN
        // arm one report poll per visit
        poll.Execute := TRUE;
        poll.ConnectionHdl := hdl;
        st := 50;
    ELSIF st = 50 THEN
        IF poll.Done THEN
            poll.Execute := FALSE;
            rep := poll.Report;
            IF poll.ReportBytes > 0 THEN
                // dchg report: dataset name + changed value only
                IF FIND(rep, 'measds') > 0 AND FIND(rep, '2.5') > 0 THEN
                    mask := mask + 16;
                END_IF;
                rset.Execute := TRUE;
                rset.ConnectionHdl := hdl;
                rset.RcbRef := rcbref;
                rset.Flags := 5;      // rptEna + GI
                st := 60;
            ELSIF waitN > 80 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 5;
            END_IF;
        END_IF;
        IF poll.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 60 THEN
        IF rset.Done THEN
            rset.Execute := FALSE;
            mask := mask + 32;
            waitN := 0;
            poll.Execute := TRUE;
            poll.ConnectionHdl := hdl;
            st := 61;
        END_IF;
        IF rset.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 61 THEN
        // reap the GI report poll
        IF poll.Done THEN
            poll.Execute := FALSE;
            rep := poll.Report;
            IF poll.ReportBytes > 0 THEN
                // GI report: dataset name + every element value
                IF FIND(rep, 'measds') > 0 AND FIND(rep, '42') > 0
                   AND FIND(rep, '2.5') > 0 THEN
                    mask := mask + 64;
                END_IF;
                rset.Execute := TRUE;
                rset.ConnectionHdl := hdl;
                rset.RcbRef := rcbref;
                rset.Flags := 0;      // disable
                st := 62;
            ELSIF waitN > 80 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                poll.Execute := TRUE;   // re-arm for next visit
                st := 61;
            END_IF;
        END_IF;
        IF poll.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 62 THEN
        IF rset.Done THEN
            rset.Execute := FALSE;
            mask := mask + 128;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 7;
        END_IF;
        IF rset.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 7 THEN
        IF dis.Done THEN
            dis.Execute := FALSE;
            mask := mask + 256;
            st := 99;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; END_IF;
    ELSIF st = 99 THEN
        // terminal
    END_IF
    con(); dis(); rget(); rset(); wr(); poll();
END_PROGRAM
