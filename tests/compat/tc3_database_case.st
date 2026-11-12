// tc3_database_case.st — e2e for tc3_database (tplc-db/SQLite).
PROGRAM tc3_database_case
VAR
    opn : FB_PLCDBOpen;
    ddl : FB_PLCDBCreateEvt;
    wr1 : FB_PLCDBWriteEvt;
    wr2 : FB_PLCDBWriteEvt;
    rd : FB_PLCDBReadEvt;
    er : FB_PLCDBGetError;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        opn.sPath := '/tmp/tplc_db_e2e.sqlite';
        opn.nCmd := 1;
        st := 1;
    ELSIF st = 1 THEN
        IF opn.bDone AND opn.hDb <> 0 THEN mask := mask OR 1; END_IF;
        opn.nCmd := 0;
        ddl.hDb := opn.hDb;
        ddl.sSql := 'CREATE TABLE IF NOT EXISTS tags (id INTEGER, val INTEGER)';
        st := 2;
    ELSIF st = 2 THEN
        ddl.nCmd := 2;
        st := 3;
    ELSIF st = 3 THEN
        IF ddl.bDone THEN mask := mask OR 2; END_IF;
        ddl.nCmd := 0;
        wr1.hDb := opn.hDb;
        wr1.sSql := 'DELETE FROM tags';
        st := 4;
    ELSIF st = 4 THEN
        wr1.nCmd := 2;
        st := 5;
    ELSIF st = 5 THEN
        IF wr1.bDone THEN mask := mask OR 4; END_IF;
        wr1.nCmd := 0;
        wr2.hDb := opn.hDb;
        wr2.sSql := 'INSERT INTO tags VALUES (2, 21), (1, 7)';
        st := 6;
    ELSIF st = 6 THEN
        wr2.nCmd := 2;
        st := 7;
    ELSIF st = 7 THEN
        IF wr2.bDone AND wr2.nChanges = 2 THEN mask := mask OR 8; END_IF;
        wr2.nCmd := 0;
        rd.hDb := opn.hDb;
        rd.sSql := 'SELECT id, val FROM tags ORDER BY id';
        st := 8;
    ELSIF st = 8 THEN
        rd.nCmd := 3;
        st := 9;
    ELSIF st = 9 THEN
        IF rd.bDone AND rd.nRows = 2 AND rd.nCols = 2 THEN
            mask := mask OR 16;
        END_IF;
        // rows: "1\t7\n2\t21\n"
        IF FIND(rd.sResult, '1') = 1 THEN mask := mask OR 32; END_IF;
        IF FIND(rd.sResult, '21') > 0 THEN mask := mask OR 64; END_IF;
        IF FIND(rd.sResult, '21') > 0 THEN mask := mask OR 128; END_IF;
        rd.nCmd := 0;
        rd.sSql := 'SELECT nope FROM tags';
        st := 10;
    ELSIF st = 10 THEN
        rd.nCmd := 3;                        // bad SQL -> error
        st := 11;
    ELSIF st = 11 THEN
        IF rd.bError THEN mask := mask OR 256; END_IF;
        rd.nCmd := 0;
        er.hDb := opn.hDb;
        st := 12;
    ELSIF st = 12 THEN
        er.nCmd := 5;                        // engine error text
        st := 13;
    ELSIF st = 13 THEN
        IF er.bDone AND FIND(er.sError, 'nope') > 0 THEN
            mask := mask OR 512;
        END_IF;
        er.nCmd := 0;
        opn.nCmd := 4;                       // close
        st := 14;
    ELSIF st = 14 THEN
        IF opn.bDone AND opn.hDb = 0 THEN mask := mask OR 1024; END_IF;
        opn.nCmd := 0;
        st := 15;
    END_IF;
    opn(); ddl(); wr1(); wr2(); rd(); er();
END_PROGRAM
