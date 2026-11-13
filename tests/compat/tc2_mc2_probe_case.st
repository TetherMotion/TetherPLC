PROGRAM Tc2Mc2ProbeCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    ax     : FB_MotionAxis;
    axM    : UDINT;
    pwr    : MC_Power;
    sp     : MC_SetPosition;
    tp     : MC_TouchProbe;
    ab     : MC_AbortTrigger;
    rpos   : MC_ReadActualPosition;
    enSig  : BOOL;
    spEx   : BOOL;
    tpEx   : BOOL;
    abEx   : BOOL;
    tpTrig : BOOL;
    abSeen : BOOL;
    abDoneSeen : BOOL;
    disarmed   : BOOL;
END_VAR

// Create one simulated axis.
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 2000;
    ax.MAX_ACCEL := 8000;
    ax.MAX_DECEL := 8000;
    ax.POS_MIN := -200000;
    ax.POS_MAX := 200000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    IF axM <> 0 THEN
        mask := mask + 1;                       // bit0: axis created
    END_IF;
    st := 1;
    prevSt := 0;
END_IF;

IF st <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := st;

enSig  := (st >= 1) AND (st <= 10);   // powered through st=10
spEx   := pulse AND (st = 2);        // preset position 1000
tpEx   := pulse AND (st = 5 OR st = 9);  // arm, later re-arm
abEx   := pulse AND (st = 7);        // abort the armed probe
tpTrig := (st >= 9);                 // trigger edge on re-arm scan

pwr(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axM);
sp(Execute := spEx, Position := 1000, Mode := FALSE, Axis := axM);
tp(Execute := tpEx, TriggerInput := tpTrig, WindowOnly := FALSE,
   FirstPosition := 0, LastPosition := 0, Axis := axM);
ab(Execute := abEx, TriggerInput := FALSE, Axis := axM);
rpos(Enable := TRUE, Axis := axM);

IF st = 1 THEN
    IF pwr.Status THEN
        mask := mask + 2;                       // bit1: powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // MC_SetPosition edge writes 1000; Done pulses on this scan
    IF sp.Done THEN
        mask := mask + 256;                     // bit8: pos preset
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    st := 4;
ELSIF st = 4 THEN
    st := 5;
ELSIF st = 5 THEN
    // probe armed this scan (Execute edge)
    st := 6;
ELSIF st = 6 THEN
    IF tp.Busy THEN
        mask := mask + 4;                       // bit2: probe armed
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    // MC_AbortTrigger Execute edge writes the abort request
    st := 8;
ELSIF st = 8 THEN
    // wait until the armed probe consumes the abort (may take an
    // extra scan over Tether IO)
    IF tp.CommandAborted AND NOT abSeen THEN
        abSeen := TRUE;
        mask := mask + 8;                       // bit3: probe aborted
    END_IF;
    IF ab.Done AND NOT abDoneSeen THEN
        abDoneSeen := TRUE;
        mask := mask + 16;                      // bit4: abort Done
    END_IF;
    IF NOT tp.Busy AND NOT disarmed THEN
        disarmed := TRUE;
        mask := mask + 32;                      // bit5: probe idle
    END_IF;
    IF abSeen AND abDoneSeen AND disarmed THEN
        st := 9;
    END_IF;
ELSIF st = 9 THEN
    // re-arm + trigger edge in the same scan: probe captures pos
    IF tp.Done THEN
        mask := mask + 64;                      // bit6: captured
        st := 10;
    END_IF;
ELSIF st = 10 THEN
    // recorded position equals the preset axis position
    IF rpos.Valid AND tp.RecordedPosition = rpos.Position
       AND tp.RecordedPosition = 1000 THEN
        mask := mask + 128;                     // bit7: pos matches
    END_IF;
    st := 11;
ELSIF st = 11 THEN
    // power off
    IF NOT pwr.Status THEN
        mask := mask + 512;                     // bit9: powered off
        st := 12;
    END_IF;
END_IF;

END_PROGRAM
