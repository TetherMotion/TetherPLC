// t19_gsched.st — FB_AtcGainSchedule breakpoint interpolation.
// Tables configured once as instance members (array call-args do not
// bind in the MIR), then X varies per call.  X=[0,50,100],
// Kp=[1,2,4], Tn=[10,20,40], Tv=[0,1,2].
// flags: b0 X=0  -> Kp=1 Seg=1 Frac=0
//        b1 X=50 -> Kp=2 Seg=1 Frac=1
//        b2 X=25 -> Kp=1.5 Tn=15 Tv=0.5 Frac=0.5
//        b3 X=75 -> Kp=3 Tn=30 Tv=1.5 Seg=2 Frac=0.5
//        b4 X=-10-> Kp=1 (clamp low)
//        b5 X=200-> Kp=4 (clamp high)
//        b6 dup-X table -> Err
//        b7 X=100-> Kp=4 Seg=2 Frac=1
PROGRAM T19
VAR
    flags AT %QB0 : BYTE;
    gs : FB_AtcGainSchedule;
    gb : FB_AtcGainSchedule;
    cyc : DINT;
END_VAR
    cyc := cyc + 1;
    IF cyc = 1 THEN
        gs.TableX[1] := 0.0;   gs.TableX[2] := 50.0;  gs.TableX[3] := 100.0;
        gs.TableKp[1] := 1.0;  gs.TableKp[2] := 2.0;  gs.TableKp[3] := 4.0;
        gs.TableTn[1] := 10.0; gs.TableTn[2] := 20.0; gs.TableTn[3] := 40.0;
        gs.TableTv[1] := 0.0;  gs.TableTv[2] := 1.0;  gs.TableTv[3] := 2.0;
        gb.TableX := gs.TableX;
        gb.TableKp := gs.TableKp;
        gb.TableTn := gs.TableTn;
        gb.TableTv := gs.TableTv;
        gb.TableX[2] := gb.TableX[1];   // duplicate X -> must flag Err
    END_IF;
    IF cyc >= 2 THEN
        flags := 0;
        gs(Execute := TRUE, X := 0.0, NPts := 3);
        IF gs.Kp = 1.0 AND gs.Seg = 1 AND gs.Frac = 0.0 THEN
            flags := flags + 1;
        END_IF;
        gs(Execute := TRUE, X := 50.0, NPts := 3);
        IF gs.Kp = 2.0 AND gs.Seg = 1 AND gs.Frac = 1.0 THEN
            flags := flags + 2;
        END_IF;
        gs(Execute := TRUE, X := 25.0, NPts := 3);
        IF gs.Kp = 1.5 AND gs.Tn = 15.0 AND gs.Tv = 0.5
           AND gs.Frac = 0.5 THEN
            flags := flags + 4;
        END_IF;
        gs(Execute := TRUE, X := 75.0, NPts := 3);
        IF gs.Kp = 3.0 AND gs.Tn = 30.0 AND gs.Tv = 1.5
           AND gs.Seg = 2 AND gs.Frac = 0.5 THEN
            flags := flags + 8;
        END_IF;
        gs(Execute := TRUE, X := -10.0, NPts := 3);
        IF gs.Kp = 1.0 THEN flags := flags + 16; END_IF;
        gs(Execute := TRUE, X := 200.0, NPts := 3);
        IF gs.Kp = 4.0 THEN flags := flags + 32; END_IF;
        gb(Execute := TRUE, X := 10.0, NPts := 3);
        IF gb.Err THEN flags := flags + 64; END_IF;
        gs(Execute := TRUE, X := 100.0, NPts := 3);
        IF gs.Kp = 4.0 AND gs.Seg = 2 AND gs.Frac = 1.0 THEN
            flags := flags + 128;
        END_IF;
    END_IF;
END_PROGRAM
