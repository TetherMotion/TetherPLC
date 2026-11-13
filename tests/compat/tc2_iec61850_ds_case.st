// tc2_iec61850_ds_case.st — IEC 61850 data-set e2e over the fake
// IED: GetDataSetDirectory + ReadDataSet on measds
// (Meas.instMag.f=42.5 MX + SetPt.setMag.f=1.0 SP), then
// WriteDataSet on writeds (SetPt.setMag.f + SetPt2.setMag.f, both SP)
// with a read-back verify, CreateDataSet/DeleteDataSet on the
// association-specific @dyn1 set, disconnect.
PROGRAM tc2_iec61850_ds_case
VAR
    con : FB_Iec61850Connect;
    dis : FB_Iec61850Disconnect;
    dsdir : FB_Iec61850DataSetDirectory;
    dsrd : FB_Iec61850DataSetRead;
    dswr : FB_Iec61850DataSetWrite;
    dscr : FB_Iec61850DataSetCreate;
    dsdell : FB_Iec61850DataSetDelete;
    st : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
    dsref : STRING[127];
    wdref : STRING[127];
    cref : STRING[127];
END_VAR
    IF st = 0 THEN
        mask := 0;
        dsref := 'tetherIEDtetherTestLD/LLN0.measds';
        wdref := 'tetherIEDtetherTestLD/LLN0.writeds';
        cref := '@dyn1';
        con.Execute := TRUE;
        con.RemoteHost := '127.0.0.1';
        con.RemotePort := 11020;
        st := 1;
    ELSIF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;
            dsdir.Execute := TRUE;
            dsdir.ConnectionHdl := hdl;
            dsdir.DataSetRef := dsref;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 2 THEN
        IF dsdir.Done THEN
            dsdir.Execute := FALSE;
            // two members: the MX instMag.f and SP setMag.f FCDAs
            IF dsdir.ElementCount = 2
               AND FIND(dsdir.Names, 'Meas') > 0
               AND FIND(dsdir.Names, 'SetPt') > 0 THEN
                mask := mask + 2;
            END_IF;
            dsrd.Execute := TRUE;
            dsrd.ConnectionHdl := hdl;
            dsrd.DataSetRef := dsref;
            st := 3;
        END_IF;
        IF dsdir.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 3 THEN
        IF dsrd.Done THEN
            dsrd.Execute := FALSE;
            // values: instMag.f=42.5, setMag.f=1.0
            IF dsrd.ElementCount = 2
               AND FIND(dsrd.Values, 'instMag') > 0
               AND FIND(dsrd.Values, '42.5') > 0
               AND FIND(dsrd.Values, 'setMag') > 0 THEN
                mask := mask + 4;
            END_IF;
            // WriteDataSet on the writable twin: stVal=9, setMag=7.25
            dswr.Execute := TRUE;
            dswr.ConnectionHdl := hdl;
            dswr.DataSetRef := wdref;
            dswr.Values := '9
7.25';
            st := 5;
        END_IF;
        IF dsrd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 5 THEN
        IF dswr.Done THEN
            dswr.Execute := FALSE;
            mask := mask + 16;
            // read back the written dataset
            dsrd.Execute := TRUE;
            dsrd.ConnectionHdl := hdl;
            dsrd.DataSetRef := wdref;
            st := 6;
        END_IF;
        IF dswr.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 6 THEN
        IF dsrd.Done THEN
            dsrd.Execute := FALSE;
            IF dsrd.ElementCount = 2
               AND FIND(dsrd.Values, '9') > 0
               AND FIND(dsrd.Values, '7.25') > 0 THEN
                mask := mask + 32;
            END_IF;
            // create the association-specific @dyn1 data set with the
            // two writable members
            dscr.Execute := TRUE;
            dscr.ConnectionHdl := hdl;
            dscr.DataSetRef := cref;
            dscr.Elements := 'tetherIEDtetherTestLD/LLN0.SetPt.setMag.f[SP]
tetherIEDtetherTestLD/LLN0.SetPt2.setMag.f[SP]';
            st := 8;
        END_IF;
        IF dsrd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 8 THEN
        IF dscr.Done THEN
            dscr.Execute := FALSE;
            mask := mask + 128;
            dsdir.Execute := TRUE;
            dsdir.ConnectionHdl := hdl;
            dsdir.DataSetRef := cref;
            st := 9;
        END_IF;
        IF dscr.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 9 THEN
        IF dsdir.Done THEN
            dsdir.Execute := FALSE;
            IF dsdir.ElementCount = 2 THEN
                mask := mask + 256;
            END_IF;
            dsdell.Execute := TRUE;
            dsdell.ConnectionHdl := hdl;
            dsdell.DataSetRef := cref;
            st := 10;
        END_IF;
        IF dsdir.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 10 THEN
        IF dsdell.Done THEN
            dsdell.Execute := FALSE;
            mask := mask + 512;
            // the deleted set must now be gone: directory errors
            dsdir.Execute := TRUE;
            dsdir.ConnectionHdl := hdl;
            dsdir.DataSetRef := cref;
            st := 11;
        END_IF;
        IF dsdell.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 11 THEN
        IF dsdir.Done THEN
            // unexpected success: the set survived deletion
            mask := 16#DEAD; st := 99;
        END_IF;
        IF dsdir.Error THEN
            dsdir.Execute := FALSE;
            mask := mask + 1024;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 12;
        END_IF;
    ELSIF st = 12 THEN
        IF dis.Done THEN
            dis.Execute := FALSE;
            mask := mask + 2048;
            st := 99;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); dsdir(); dsrd(); dswr(); dscr(); dsdell();
END_PROGRAM
