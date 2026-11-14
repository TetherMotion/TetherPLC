// o40_cam.st — FB_AtcCam: trapezoid (0,0)-(10,100)-(20,100)-(30,0),
// X=2n sweep n=1..20. Y interp: X=6->60, plateau 100 for X 10..20,
// X=22->80, X>=30 Sat + Y=0. flags %QB0: expect 0x3f.
PROGRAM O40
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    cam : FB_AtcCam;
    n : DINT;
    x : LREAL;
END_VAR
    n := n + 1;
    x := DINT_TO_LREAL(n) * 2.0;
    cam.PTS[1].X := 0.0;  cam.PTS[1].Y := 0.0;
    cam.PTS[2].X := 10.0; cam.PTS[2].Y := 100.0;
    cam.PTS[3].X := 20.0; cam.PTS[3].Y := 100.0;
    cam.PTS[4].X := 30.0; cam.PTS[4].Y := 0.0;
    cam(X := x, NPTS := 4, Enable := TRUE);
    IF n = 3 AND cam.Y = 60.0 AND NOT cam.Sat THEN ok0 := TRUE;
    END_IF;
    IF n = 6 AND cam.Y = 100.0 THEN ok1 := TRUE; END_IF;
    IF n = 11 AND cam.Y = 80.0 THEN ok2 := TRUE; END_IF;
    IF n >= 16 AND n <= 20 AND cam.Sat AND cam.Y = 0.0 THEN
        ok3 := TRUE;
    END_IF;
    IF n = 8 AND cam.Y = 100.0 THEN ok4 := TRUE; END_IF;
    IF n >= 17 AND n <= 20 AND cam.Sat THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
