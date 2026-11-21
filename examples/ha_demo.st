// ha_demo.st — interactive Home Assistant demo for FB_HaBridge.
//
// Simulates a boiler: HA sets the temperature setpoint, toggles the
// heater, and presses a "bump" button that kicks a load disturbance.
// Live PV and controller output publish back to HA as sensors.
//
// Entities appear automatically in HA via MQTT discovery:
//   sensor.boiler_temp   — plant PV (degC)
//   sensor.heater_out    — PID manipulated output (%)
//   switch.heater        — enables/disables the control loop
//   number.temp_sp       — setpoint (degC), AutoWr
//   button.bump          — fires a 5 s disturbance pulse (event)
//
// Point br.Host / br.Port at your broker:
//   - Mosquitto:        sudo apt install mosquitto  (listener 1883)
//   - HA Mosquitto add-on: broker runs inside Home Assistant
//
// Build:
//   tplcc build --library --name HaBridge libraries/homeassistant/ha_bridge.st -o ha_bridge.tplc
//   tplcc build --library --name Atc_TempCtrl libraries/tempctrl/atc_tempctrl.st -o atc.tplc
//   tplcc build examples/ha_demo.st --lib ha_bridge.tplc --lib atc.tplc \
//       --task-period-ns 100000000 -o ha_demo.tplc   # 100 ms scan = ct
PROGRAM ha_demo
VAR
    br    : FB_HaBridge;
    plant : FB_AtcPlant;
    pid   : FB_AtcPid;
    ct    : LREAL := 0.1;          // 100 ms scan
    bumpT : LREAL := 0.0;          // disturbance pulse timer (s)
    bumpEv: BOOL;
END_VAR

    // ---------- MQTT bridge ----------
    br.Execute  := TRUE;
    br.CycleTime := ct;
    br.Dev      := 'boiler';               // topic root: tether/boiler/...
    br.Host     := '192.168.1.10';         // <-- YOUR BROKER IP
    br.Port     := 1883;
    br.KeepAlive := 60;
    br.Clean    := TRUE;
    br.BackoffMin := 500;
    br.BackoffMax := 5000;
    br.HbS      := 5;                      // state heartbeat every 5 s

    // ch1: boiler temperature (PLC -> HA)
    br.CHAN[1].En    := TRUE;
    br.CHAN[1].Kind  := 0;                 // sensor
    br.CHAN[1].ObjId := 'boiler_temp';
    br.CHAN[1].Name  := 'Boiler temperature';
    br.CHAN[1].Unit  := 'degC';
    br.CHAN[1].DevCls:= 'temperature';
    br.CHAN[1].MinV  := 0.0;
    br.CHAN[1].MaxV  := 120.0;

    // ch2: heater enable (HA -> PLC, BVal readable)
    br.CHAN[2].En    := TRUE;
    br.CHAN[2].Kind  := 2;                 // switch
    br.CHAN[2].ObjId := 'heater';
    br.CHAN[2].Name  := 'Heater';

    // ch3: temperature setpoint (both ways, AutoWr latches back)
    br.CHAN[3].En    := TRUE;
    br.CHAN[3].Kind  := 3;                 // number
    br.CHAN[3].ObjId := 'temp_sp';
    br.CHAN[3].Name  := 'Temperature setpoint';
    br.CHAN[3].Unit  := 'degC';
    br.CHAN[3].MinV  := 20.0;
    br.CHAN[3].MaxV  := 110.0;
    br.CHAN[3].StepV := 0.5;
    br.CHAN[3].AutoWr:= TRUE;
    br.CHAN[3].Value := 60.0;

    // ch4: bump button (HA -> PLC event)
    br.CHAN[4].En    := TRUE;
    br.CHAN[4].Kind  := 4;                 // button
    br.CHAN[4].ObjId := 'bump';
    br.CHAN[4].Name  := 'Load bump';

    // ch5: controller output (PLC -> HA)
    br.CHAN[5].En    := TRUE;
    br.CHAN[5].Kind  := 0;                 // sensor
    br.CHAN[5].ObjId := 'heater_out';
    br.CHAN[5].Name  := 'Heater output';
    br.CHAN[5].Unit  := '%';
    br.CHAN[5].MinV  := 0.0;
    br.CHAN[5].MaxV  := 100.0;

    // ---------- event: HA button press -> 5 s disturbance ----------
    IF br.CHAN[4].Event THEN               // 1-scan pulse on press
        bumpT := 5.0;
    END_IF;
    IF bumpT > 0.0 THEN
        bumpT := bumpT - ct;
    END_IF;
    bumpEv := bumpT > 0.0;

    // ---------- boiler process + PID ----------
    plant.Execute := TRUE;
    plant.U       := pid.Out;
    plant.K       := 0.8;                  // degC per %
    plant.Tau1    := 25.0;
    plant.Theta   := 2.0;
    plant.Pv0     := 20.0;
    plant.Taamb   := 20.0;
    plant.TauAmb  := 120.0;
    plant.NoiseAmp:= 0.05;
    plant.DistU   := SEL(bumpEv, 35.0, 0.0); // "someone opened a door"
    plant.CycleTime:= ct;
    plant();

    pid.Execute   := br.CHAN[2].BVal;      // heater switch gates control
    pid.Setpoint  := br.CHAN[3].Value;     // HA number -> SP
    pid.Actual    := plant.PV;
    pid.Kp        := 4.0;
    pid.Tn        := 20.0;
    pid.Tv        := 0.0;
    pid.OutMin    := 0.0;
    pid.OutMax    := 100.0;
    pid.CycleTime := ct;
    pid();

    // ---------- publish live values ----------
    br.CHAN[1].Value := plant.PV;
    br.CHAN[5].Value := pid.Out;

    br();
END_PROGRAM
