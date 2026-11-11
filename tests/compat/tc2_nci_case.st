PROGRAM Tc2NciCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    ax     : FB_MotionAxis;
    axX     : UDINT;
    axY     : UDINT;
    axZ     : UDINT;
    pX     : MC_Power;
    pY     : MC_Power;
    pZ     : MC_Power;
    ln     : FB_NciLine;
    estp   : FB_NciEStop;
    rX     : MC_ReadActualPosition;
    rY     : MC_ReadActualPosition;
    rZ     : MC_ReadActualPosition;
    enSig  : BOOL;
    lnEx   : BOOL;
    estpEx : BOOL;
    lnX    : DINT;
    lnY    : DINT;
    lnZ    : DINT;
END_VAR

// Create the three group axes once.
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 200000;
    ax.MAX_ACCEL := 800000;
    ax.MAX_DECEL := 800000;
    ax.POS_MIN := -2000000;
    ax.POS_MAX := 2000000;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axX := ax.AXIS; END_IF;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axY := ax.AXIS; END_IF;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axZ := ax.AXIS; END_IF;
    IF axX <> 0 AND axY <> 0 AND axZ <> 0 THEN
        mask := mask + 1;                       // bit0: 3 axes
    END_IF;
    st := 1;
    prevSt := 0;
END_IF;

IF st <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := st;

enSig  := (st >= 1) AND (st <= 5);
lnEx   := (st = 2) OR (st = 4);    // move1 target, move2 home
estpEx := pulse AND (st = 6);
IF st <= 3 THEN
    lnX := 1000; lnY := 600; lnZ := 300;   // first segment
ELSE
    lnX := 0;    lnY := 0;   lnZ := 0;     // return to origin
END_IF;

pX(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
   Axis := axX);
pY(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
   Axis := axY);
pZ(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
   Axis := axZ);
ln(Execute := lnEx, PosX := lnX, PosY := lnY, PosZ := lnZ,
   Velocity := 2000, CycleUs := 10000,
   AxisX := axX, AxisY := axY, AxisZ := axZ);
estp(Execute := estpEx, AxisX := axX, AxisY := axY, AxisZ := axZ);
rX(Enable := TRUE, Axis := axX);
rY(Enable := TRUE, Axis := axY);
rZ(Enable := TRUE, Axis := axZ);

IF st = 1 THEN
    IF pX.Status AND pY.Status AND pZ.Status THEN
        mask := mask + 2;                    // bit1: group powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // coordinated linear move to (1000,600,300)
    IF ln.Done THEN
        mask := mask + 4;                    // bit2: move1 done
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    // every axis arrived at its target coordinate (CSP == commanded).
    // the actual position lags the streamed target by a scan or two
    // depending on the backend, so wait for it to converge before
    // asserting the endpoint bits and moving on.
    IF rX.Position = 1000 AND rY.Position = 600 AND
       rZ.Position = 300 THEN
        mask := mask + 8;                    // bit3: endpoints hit
        // path length = sqrt(1e6+3.6e5+9e4)=sqrt(1.45e6)=1204.16
        IF ln.PathLen >= 1200 AND ln.PathLen <= 1210 THEN
            mask := mask + 16;               // bit4: path length
        END_IF;
        st := 4;
    END_IF;
ELSIF st = 4 THEN
    // second coordinated move back to the origin
    IF ln.Done THEN
        mask := mask + 32;                   // bit5: move2 done
        st := 5;
    END_IF;
ELSIF st = 5 THEN
    // wait for the positions to converge on the origin (same
    // backend-lag reasoning as st=3)
    IF rX.Position = 0 AND rY.Position = 0 AND
       rZ.Position = 0 THEN
        mask := mask + 64;                   // bit6: back at origin
        st := 6;
    END_IF;
ELSIF st = 6 THEN
    // group E-stop removes power from all three axes
    IF estp.Status THEN
        mask := mask + 128;                  // bit7: all disabled
        st := 7;
    END_IF;
END_IF;

END_PROGRAM
