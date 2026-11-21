// ha_entity_case.st — FB_HaEntity e2e: one switch entity on the
// fake broker.  HA 'ON' command is injected after the entity's
// command subscription appears; AutoWr latches BVal so OutBVal
// must go TRUE.  mask %QD0 bits (expect 0x0f):
//   b0 Connected          b2 Cmd=1 (ON) seen
//   b1 TxCnt>=3 (discovery+avail+state)   b3 OutBVal TRUE
PROGRAM ha_entity_case
VAR
    en : FB_HaEntity;
    st : INT;
    mask AT %QD0 : UDINT;
    cmdSeen : BOOL;
    latched : BOOL;
END_VAR
    en.Execute := TRUE;
    en.CycleTime := 0.01;
    en.En := TRUE;
    en.Dev := 'ent1';
    en.Host := '127.0.0.1';
    en.Port := 11889;
    en.ClientId := 'ha-entity-e2e';
    en.ConnectMs := 3000;
    en.Kind := 2;                 // switch
    en.ObjId := 'pump';
    en.Name := 'Circ pump';
    en.DevCls := 'switch';
    en.AutoWr := TRUE;
    en.BVal := FALSE;

    en();

    IF st = 0 THEN
        mask := 0;
        IF en.Connected THEN
            mask := mask + 1;
            st := 1;
        END_IF;
        IF st < 90 AND en.Err THEN st := 90; END_IF;
    END_IF;
    IF st = 1 THEN
        // discovery config + avail + first state publish
        IF en.TxCnt >= 3 THEN
            mask := mask + 2;
            st := 2;
        END_IF;
        IF st < 90 AND en.Err THEN st := 90; END_IF;
    END_IF;
    IF st >= 2 AND st < 90 THEN
        IF en.Cmd = 1 AND NOT cmdSeen THEN
            mask := mask + 4;
            cmdSeen := TRUE;
        END_IF;
        IF cmdSeen AND en.OutBVal AND NOT latched THEN
            mask := mask + 8;
            latched := TRUE;
        END_IF;
        IF mask >= 15 THEN
            st := 6;
        END_IF;
    END_IF;
    IF st >= 90 THEN
        mask := 16#DEAD;
    END_IF;
END_PROGRAM
