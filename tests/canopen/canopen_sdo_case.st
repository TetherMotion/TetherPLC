// canopen_sdo_case.st — tplc-canopen e2e against tether_io_ec_server
// (TPLC_CANOPEN_IO, --session-role technician): CiA 301 SDO upload/
// download through FB_CanOpen over the machine.sdo.* functions.
// Exercises: service state, expedited-size read of the seeded
// 0x6041/0 statusword, verified write + readback of 0x6060/0,
// CoE abort propagation on a missing object, unverified write,
// and invalid-op rejection.  One FB call per scan (each call is a
// synchronous IO round-trip).
PROGRAM canopen_sdo_case
VAR
    co : FB_CanOpen;
    st : INT;
    waitN : UDINT;
    mask AT %QD0 : UDINT;
    ok1 : BOOL;
    ok2 : BOOL;
    ok3 : BOOL;
    ok4 : BOOL;
    ok5 : BOOL;
    ok6 : BOOL;
    ok7 : BOOL;
    ok8 : BOOL;
END_VAR
    IF st = 0 THEN
        mask := 0;
        co.OP := 0;
        co.SLAVE := 0;
        co.INDEX := 0;
        co.SUB := 0;
        co.VERIFY := 0;
        co.DLEN := 0;
        st := 1;
    ELSIF st = 1 THEN
        // OP 3: service state — endpoint connected, catalog found
        co.OP := 3;
        co();
        ok1 := co.STATUS = 0;
        IF ok1 THEN mask := mask + 1; END_IF;
        // arm read of 0x6041/0 (seeded {0x50,0x02})
        co.OP := 1;
        co.SLAVE := 0;
        co.INDEX := 16#6041;
        co.SUB := 0;
        co.DLEN := 0;   // 0 -> service requests full 256-byte cap
        st := 2;
    ELSIF st = 2 THEN
        // execute the read armed in st=1 (synchronous round-trip)
        co();
        ok2 := co.STATUS = 0 AND co.DLEN = 2 AND
               co.DATA[0] = 16#50 AND co.DATA[1] = 16#02 AND
               co.ABORT = 0;
        IF ok2 THEN mask := mask + 2; END_IF;
        // arm verified write of 0x03 to 0x6060/0
        co.OP := 2;
        co.SLAVE := 0;
        co.INDEX := 16#6060;
        co.SUB := 0;
        co.DATA[0] := 16#03;
        co.DLEN := 1;
        co.VERIFY := 1;
        st := 3;
    ELSIF st = 3 THEN
        co();
        ok3 := co.STATUS = 0 AND co.ABORT = 0;
        IF ok3 THEN mask := mask + 4; END_IF;
        // read back 0x6060/0
        co.OP := 1;
        co.SLAVE := 0;
        co.INDEX := 16#6060;
        co.SUB := 0;
        co.DLEN := 0;
        co.VERIFY := 0;
        st := 4;
    ELSIF st = 4 THEN
        co();
        ok4 := co.STATUS = 0 AND co.DLEN = 1 AND co.DATA[0] = 16#03;
        IF ok4 THEN mask := mask + 8; END_IF;
        // read a missing object -> CoE abort 0x06020000
        co.OP := 1;
        co.SLAVE := 0;
        co.INDEX := 16#9999;
        co.SUB := 0;
        co.DLEN := 0;
        st := 5;
    ELSIF st = 5 THEN
        co();
        ok5 := co.STATUS = -4 AND co.ABORT = 16#06020000;
        IF ok5 THEN mask := mask + 16; END_IF;
        // unverified write of 0x07 to 0x6060/0 — delayed: the server
        // rate-limits sdo.write to one per 50 ms of *real* time and the
        // sim replays scans far faster than that
        waitN := 0;
        st := 60;
    ELSIF st = 60 THEN
        waitN := waitN + 1;
        IF waitN >= 8000 THEN
            co.OP := 2;
            co.SLAVE := 0;
            co.INDEX := 16#6060;
            co.SUB := 0;
            co.DATA[0] := 16#07;
            co.DLEN := 1;
            co.VERIFY := 0;
            st := 6;
        END_IF;
    ELSIF st = 6 THEN
        co();
        ok6 := co.STATUS = 0 AND co.ABORT = 0;
        IF ok6 THEN mask := mask + 32; END_IF;
        // final read confirms the unverified write landed
        co.OP := 1;
        co.SLAVE := 0;
        co.INDEX := 16#6060;
        co.SUB := 0;
        co.DLEN := 0;
        co.VERIFY := 0;
        st := 7;
    ELSIF st = 7 THEN
        co();
        ok7 := co.STATUS = 0 AND co.DLEN = 1 AND co.DATA[0] = 16#07;
        IF ok7 THEN mask := mask + 64; END_IF;
        // invalid op -> E_PARAM-style rejection
        co.OP := 9;
        st := 8;
    ELSIF st = 8 THEN
        co();
        ok8 := co.STATUS = -1 AND co.ERRID = 1;
        IF ok8 THEN mask := mask + 128; END_IF;
        co.OP := 0;
        st := 9;
    END_IF
END_PROGRAM
