PROGRAM Tc3McGroupCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    cur    : INT;
    pulse  : BOOL;
    started : BOOL;
    grp    : UDINT;
    grpBad : UDINT;
    ax7    : UDINT;
    ax9    : UDINT;
    addA   : MC_AddAxisToGroup;
    addB   : MC_AddAxisToGroup;
    remA   : MC_RemoveAxisFromGroup;
    ungrp  : MC_UngroupAllAxes;
    enA    : MC_GroupEnable;
    enBad  : MC_GroupEnable;
    disA   : MC_GroupDisable;
    rstA   : MC_GroupReset;
    rs     : MC_GroupReadStatus;
    re     : MC_GroupReadError;
    so     : MC_GroupSetOverride;
    ss     : MC_GroupSetSetpointSource;
    ct     : MC_SetCoordinateTransform;
    ident  : UDINT;
    mg     : FB_McGroup;
    memOk  : BOOL;
END_VAR

cur := st;
pulse := (cur <> prevSt) OR NOT started;
started := TRUE;
prevSt := cur;

grp := 1;
grpBad := 2;
ax7 := 7;
ax9 := 9;
ident := UDINT_TO_IDENTINGROUP(0);
memOk := FALSE;

// Level-triggered readers: called every scan.
rs(Enable := TRUE, AxesGroup := grp);
re(Enable := TRUE, AxesGroup := grp);

// st0: add axis 7 to group 1 (ident 0)
IF cur = 0 THEN
    addA(Execute := pulse, IdentInGroup := ident,
         AxesGroup := grp, Axis := ax7);
    IF pulse AND addA.Done AND NOT addA.Error THEN
        mask := mask + 1;                       // bit0: add axis
    END_IF;
    st := 1;
// st1: status readback -> disabled, valid
ELSIF cur = 1 THEN
    IF rs.Valid AND rs.GroupDisabled THEN
        mask := mask + 2;                       // bit1: read status
    END_IF;
    st := 2;
// st2: enable group -> standby
ELSIF cur = 2 THEN
    enA(Execute := pulse, AxesGroup := grp);
    IF pulse AND enA.Done AND NOT enA.Error THEN
        mask := mask + 4;                       // bit2: enable done
    END_IF;
    st := 3;
// st3: status now standby + group error read valid
ELSIF cur = 3 THEN
    IF rs.GroupStandby AND re.Valid AND re.GroupErrorId = 0 THEN
        mask := mask + 8;                       // bit3: standby+err
    END_IF;
    st := 4;
// st4: enable empty group 2 -> error
ELSIF cur = 4 THEN
    enBad(Execute := pulse, AxesGroup := grpBad);
    IF pulse AND enBad.Error THEN
        mask := mask + 16;                      // bit4: empty enable err
    END_IF;
    st := 5;
// st5: override 150%
ELSIF cur = 5 THEN
    so(Execute := pulse, Enable := TRUE, VelFactor := 1.5,
       AxesGroup := grp);
    IF pulse AND so.Done AND
       ABS(so.OverrideActual - 1.5) < 0.001 THEN
        mask := mask + 32;                      // bit5: override
    END_IF;
    st := 6;
// st6: setpoint source MCS (2)
ELSIF cur = 6 THEN
    ss(Execute := pulse, SetpointSource := 2, AxesGroup := grp);
    IF pulse AND ss.Done AND NOT ss.Error THEN
        mask := mask + 64;                      // bit6: setpoint src
    END_IF;
    st := 7;
// st7: coordinate transform 3 -> active
ELSIF cur = 7 THEN
    ct(Execute := pulse, CoordTransform := 3, AxesGroup := grp);
    IF pulse AND ct.Done AND ct.Active AND NOT ct.Error THEN
        mask := mask + 128;                     // bit7: coord transform
    END_IF;
    st := 8;
// st8: membership readback via raw FB_McGroup OP10
ELSIF cur = 8 THEN
    mg.OP := 10; mg.GRP := grp; mg();
    IF mg.STATUS = 0 AND mg.NAXES = 1 AND mg.AXES[0] = 7 THEN
        memOk := TRUE;
        mask := mask + 256;                     // bit8: membership
    END_IF;
    st := 9;
// st9: disable group (membership changes need disabled state)
ELSIF cur = 9 THEN
    disA(Execute := pulse, AxesGroup := grp);
    IF pulse AND disA.Done THEN st := 10; END_IF;
// st10: add axis 9, remove axis 7 -> still 1 member (9)
ELSIF cur = 10 THEN
    addB(Execute := pulse, IdentInGroup := ident,
         AxesGroup := grp, Axis := ax9);
    remA(Execute := pulse, IdentInGroup := ident,
          AxesGroup := grp, Axis := ax7);
    IF pulse AND addB.Done AND remA.Done THEN st := 11; END_IF;
// st11: verify swap + ungroup all
ELSIF cur = 11 THEN
    mg.OP := 10; mg.GRP := grp; mg();
    IF mg.STATUS = 0 AND mg.NAXES = 1 AND mg.AXES[0] = 9 THEN
        mask := mask + 512;                     // bit9: swap member
    END_IF;
    ungrp(Execute := pulse, AxesGroup := grp);
    IF pulse AND ungrp.Done AND NOT ungrp.Error THEN
        mask := mask + 1024;                    // bit10: ungroup
    END_IF;
    st := 12;
// st12: group reset on fresh group
ELSIF cur = 12 THEN
    rstA(Execute := pulse, AxesGroup := grpBad);
    IF pulse AND rstA.Done THEN
        mask := mask + 2048;                    // bit11: reset
    END_IF;
    st := 13;
ELSIF cur = 13 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
