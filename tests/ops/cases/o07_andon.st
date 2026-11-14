// o07_andon.st — FB_AtcAndon: combinational priority mapping.
//   n=1 Run        -> Green
//   n=2 +Warning   -> Amber, Green off
//   n=3 +Alarm     -> Red+Buzz, Amber still (Warn held)
//   n=5 all clear +Changeover -> Blue, Green suppressed
//   n=6 Maintenance -> Amber (no Run)
// Sticky bits per output seen in its expected phase.
PROGRAM O07
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    an : FB_AtcAndon;
    n : DINT;
END_VAR
    n := n + 1;
    an(Run := n <= 4 OR n = 5, Warning := n = 2 OR n = 3,
       Alarm := n = 3, Fault := FALSE,
       Maintenance := n = 6, Changeover := n = 5);
    IF n = 1 AND an.Green AND NOT an.Red AND NOT an.Amber
       THEN ok0 := TRUE; END_IF;
    IF n = 2 AND an.Amber AND NOT an.Green AND NOT an.Red
       THEN ok1 := TRUE; END_IF;
    IF n = 3 AND an.Red AND an.Buzzer AND an.Amber
       THEN ok2 := TRUE; END_IF;
    IF n = 5 AND an.Blue AND NOT an.Green AND NOT an.Red
       THEN ok3 := TRUE; END_IF;
    IF n = 6 AND an.Amber AND NOT an.Green THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
