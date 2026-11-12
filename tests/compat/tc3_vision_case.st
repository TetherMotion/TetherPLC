// tc3_vision_case.st — Tc3_Vision e2e: camera-open handshake,
// acquire-image not-open guard, job dispatch — all honest
// engine-absent paths.
PROGRAM tc3_vision_case
VAR
    mask AT %QD0 : UDINT;
    cam : FB_VN_GevCamera;
    acq : FB_VN_AcquireImage;
    job : FB_VN_JobControl;
    st : INT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        cam.Open := FALSE;
        acq.Trigger := FALSE;
        job.Execute := FALSE;
        acq.CameraOpen := FALSE;
        cam.IpAddress := '192.168.1.10';
        cam.Port := 3956;
        st := 1;
    ELSIF st = 1 THEN
        // acquire with no open channel -> not-open guard
        acq.Trigger := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        IF acq.Error AND acq.ErrorId = 5 AND NOT acq.ImageReady THEN
            mask := mask OR 1;
            acq.Trigger := FALSE;
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        cam.Open := TRUE;          // valid address -> handshake
        st := 4;
    ELSIF st = 4 THEN
        IF cam.Busy THEN mask := mask OR 2; END_IF;
        IF cam.Error AND cam.ErrorId = 1 AND NOT cam.Opened THEN
            mask := mask OR 4;      // engine-absent on open
            cam.Open := FALSE;
            st := 5;
        END_IF;
    ELSIF st = 5 THEN
        cam.IpAddress := '';       // bad address -> param error
        cam.Open := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        IF cam.Error AND cam.ErrorId = 2 THEN
            mask := mask OR 8;
            cam.Open := FALSE;
            st := 7;
        END_IF;
    ELSIF st = 7 THEN
        job.JobType := 42;         // valid job -> handshake
        job.Execute := TRUE;
        st := 8;
    ELSIF st = 8 THEN
        IF job.Error AND job.ErrorId = 1 AND NOT job.ResultValid THEN
            mask := mask OR 16;
            job.Execute := FALSE;
            st := 9;
        END_IF;
    ELSIF st = 9 THEN
        job.JobType := 0;          // bad job id -> param error
        job.Execute := TRUE;
        st := 10;
    ELSIF st = 10 THEN
        IF job.Error AND job.ErrorId = 3 THEN
            mask := mask OR 32;
            st := 11;
        END_IF;
    END_IF;

    cam();
    acq();
    job();
END_PROGRAM
