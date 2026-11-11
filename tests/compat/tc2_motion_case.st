PROGRAM Tc2MotionCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    ax     : FB_MotionAxis;
    axH    : UDINT;
    aStatus : DINT;
    aTgt   : DINT;
    aPos   : DINT;
    aVel   : DINT;
    sst    : DINT;
    gotEn  : BOOL;
END_VAR

// Deterministic simulated CiA402 axis exercised through the
// FB_MotionAxis builtin (tplc-motion service). One poll per scan
// advances dynamics; each state performs synchronous ops.
// DS402 object indices: 0x6040 cw, 0x6060 mode, 0x607A pos target,
// 0x60FF vel target. Controlword 0x001F = enable op + new setpoint.
ax.OP := 3;   // poll: advance all axes by the sim clock
ax();

IF st = 0 THEN
    // create: units/rev 1000, vmax 1000 u/s, accel/decel 5000 u/s^2
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 1000;
    ax.MAX_ACCEL := 5000;
    ax.MAX_DECEL := 5000;
    ax.POS_MIN := -100000;
    ax.POS_MAX := 100000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1;
    ax();
    IF ax.AXIS <> 0 AND ax.STATUS = 0 THEN
        axH := ax.AXIS;
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 THEN
    // mode = profile position (1)
    ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 1; ax();
    IF ax.STATUS = 0 THEN mask := mask + 2; END_IF;
    st := 2;
ELSIF st = 2 THEN
    // target position = 500
    ax.OP := 5; ax.INDEX := 16#607A; ax.VALUE := 500; ax();
    IF ax.STATUS = 0 THEN mask := mask + 4; END_IF;
    st := 3;
ELSIF st = 3 THEN
    // controlword 0x001F: enable operation + new setpoint edge
    ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 31; ax();
    IF ax.STATUS = 0 THEN mask := mask + 8; END_IF;
    st := 4;
ELSIF st = 4 THEN
    // wait for operation-enabled then the PP move to finish
    ax.OP := 6; ax.AXIS := axH; ax();
    aStatus := ax.STATUS;
    aTgt := ax.TARGET_REACHED;
    aPos := ax.POS_ACTUAL; aVel := ax.VEL_ACTUAL;
    sst := aStatus;
    IF sst = 39 AND NOT gotEn THEN
        gotEn := TRUE;
        mask := mask + 16;   // reached CiA402 OperationEnabled (0x27)
    END_IF;
    IF aTgt = 1 AND aPos >= 497 AND aPos <= 503 THEN
        mask := mask + 32;   // target reached within follow window
        st := 5;
    END_IF;
ELSIF st = 5 THEN
    // velocity mode: mode=2, target_vel=800 -> vel_actual ~800
    ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 2; ax();
    ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 800; ax();
    st := 6;
ELSIF st = 6 THEN
    // wait for velocity to settle near 800 and position to move
    ax.OP := 6; ax.AXIS := axH; ax();
    aVel := ax.VEL_ACTUAL; aPos := ax.POS_ACTUAL;
    IF aVel >= 790 AND aVel <= 810 THEN
        mask := mask + 64;
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    // error paths: bad object index read, bad mode write
    ax.OP := 4; ax.INDEX := 16#9999; ax();
    IF ax.STATUS < 0 THEN mask := mask + 128; END_IF;
    ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 99; ax();
    IF ax.STATUS < 0 THEN mask := mask + 256; END_IF;
    // info on an invalid handle
    ax.OP := 6; ax.AXIS := 9999; ax();
    IF ax.STATUS < 0 THEN mask := mask + 512; END_IF;
    st := 8;
ELSIF st = 8 THEN
    // restore a valid mode, then destroy the axis
    ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 1; ax.AXIS := axH; ax();
    ax.OP := 2; ax.AXIS := axH; ax();
    IF ax.STATUS = 0 THEN mask := mask + 1024; END_IF;
    st := 9;
ELSIF st = 9 THEN
    // info on the destroyed handle must fail
    ax.OP := 6; ax.AXIS := axH; ax();
    IF ax.STATUS < 0 THEN mask := mask + 2048; END_IF;
    st := 10;
END_IF;

END_PROGRAM
