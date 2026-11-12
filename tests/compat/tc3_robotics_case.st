// tc3_robotics_case.st — mxAutomation/uniVAL surface e2e:
// connect handshake, move-dispatch validation, engine-absent.
PROGRAM tc3_robotics_case
VAR
    mask AT %QD0 : UDINT;
    grp : FB_RobotGroup;
    cmd : FB_RobotCommand;
    st : INT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        grp.Connect := FALSE;
        cmd.Execute := FALSE;
        grp.Controller := 'krc4-cell';
        grp.RobotCount := 1;
        cmd.GroupConnected := FALSE;
        cmd.MoveType := 1;
        cmd.Velocity := 100.0;
        st := 1;
    ELSIF st = 1 THEN
        // move with no group connection -> not-connected guard
        cmd.Execute := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        IF cmd.Error AND cmd.ErrorId = 5 AND NOT cmd.ReachedTarget THEN
            mask := mask OR 1;
            cmd.Execute := FALSE;
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        grp.Connect := TRUE;
        st := 4;
    ELSIF st = 4 THEN
        IF grp.Busy THEN mask := mask OR 2; END_IF;
        IF grp.Error AND grp.ErrorId = 1 AND NOT grp.Connected THEN
            mask := mask OR 4;      // license/engine absent
            grp.Connect := FALSE;
            st := 5;
        END_IF;
    ELSIF st = 5 THEN
        grp.Controller := '';      // bad address -> param error
        grp.Connect := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        IF grp.Error AND grp.ErrorId = 2 THEN
            mask := mask OR 8;
            grp.Connect := FALSE;
            st := 7;
        END_IF;
    ELSIF st = 7 THEN
        // connected-flag simulated high -> engine-absent path
        cmd.GroupConnected := TRUE;
        cmd.MoveType := 2;
        cmd.X := 100.0; cmd.Y := 0.0; cmd.Z := 50.0;
        cmd.Execute := TRUE;
        st := 8;
    ELSIF st = 8 THEN
        IF cmd.Error AND cmd.ErrorId = 1 AND NOT cmd.ReachedTarget THEN
            mask := mask OR 16;
            cmd.Execute := FALSE;
            st := 9;
        END_IF;
    ELSIF st = 9 THEN
        cmd.MoveType := 9;         // invalid move -> param error
        cmd.Execute := TRUE;
        st := 10;
    ELSIF st = 10 THEN
        IF cmd.Error AND cmd.ErrorId = 3 THEN
            mask := mask OR 32;
            st := 11;
        END_IF;
    END_IF;

    grp();
    cmd();
END_PROGRAM
