// hist_case.st — historian feedstock: n counts scans, ramp is a
// real-valued ramp, stage is a coarse bucket. All are plain VARs so
// they land in the package symmap and HIST_CONF can resolve them
// by name. flags %QB0 bit0 sets once n >= 50 (sanity marker).
PROGRAM HistCase
VAR
    n : DINT;
    ramp : LREAL;
    stage : INT;
    flags AT %QB0 : BYTE;
END_VAR
    n := n + 1;
    ramp := DINT_TO_LREAL(n) * 0.5;
    stage := DINT_TO_INT(n / 16);
    flags := 0;
    IF n >= 50 THEN flags := 1; END_IF;
END_PROGRAM
