// tc2_packml.st — PackML (ISA-TR88) unit/machine state machine
// surface — pure-ST, no service or hardware.
//
// FB_PackML implements the OMAC PackML unit-state model: the 17
// nominal states driven by the PackML commands.  State constants:
//   1 Idle        2 Starting      3 Execute     4 Completing
//   5 Complete    6 Resetting     7 Holding     8 Held
//   9 Unholding  10 Suspending   11 Suspended  12 Unsuspending
//  13 Aborting   14 Aborted      15 Clearing   16 Stopping
//  17 Stopped
// Command constants:
//   0 None   1 Start  2 Stop   3 Reset   4 Hold   5 Unhold
//   6 Suspend  7 Unsuspend  8 Abort  9 Clear
// The machine follows the standard transition table — *ing
// wait-states self-advance when Busy clears.

// ---- FB_PackML: ISA-TR88 unit-state machine ----
FUNCTION_BLOCK FB_PackML
VAR_INPUT
    Execute : BOOL;
    Command : UDINT;        // 0..9 (PML cmd, see header)
    Busy : BOOL;            // external work-in-progress for *ing states
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    State : UDINT;          // 1..17 (PML state, see header)
    InExecute : BOOL;
    IsHeld : BOOL;
    IsStopped : BOOL;
    IsAborted : BOOL;
END_VAR
VAR
    cur : UDINT;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN RETURN; END_IF;
    IF cur = 0 THEN cur := 1; END_IF;   // init -> Idle

    // ---- command handling on stable states ----
    IF Command = 8 AND cur <> 14 THEN                  // Abort
        cur := 13;                                      // -> Aborting
    ELSIF Command = 2 AND                               // Stop
         (cur = 1 OR cur = 3 OR cur = 5 OR cur = 8 OR
          cur = 11 OR cur = 17) THEN
        cur := 16;                                      // -> Stopping
    ELSIF Command = 9 AND (cur = 14 OR cur = 17) THEN  // Clear
        cur := 15;                                      // -> Clearing
    ELSIF Command = 3 AND cur = 1 THEN                  // Reset
        cur := 6;                                       // -> Resetting
    ELSIF Command = 1 AND cur = 1 THEN                  // Start
        cur := 2;                                       // -> Starting
    ELSIF Command = 4 AND cur = 3 THEN                  // Hold
        cur := 7;                                       // -> Holding
    ELSIF Command = 5 AND cur = 8 THEN                  // Unhold
        cur := 9;                                       // -> Unholding
    ELSIF Command = 6 AND cur = 3 THEN                  // Suspend
        cur := 10;                                      // -> Suspending
    ELSIF Command = 7 AND cur = 11 THEN                 // Unsuspend
        cur := 12;                                      // -> Unsuspending
    END_IF;

    // ---- *ing wait-states self-advance when Busy clears ----
    IF NOT Busy THEN
        CASE cur OF
            2:  cur := 3;   // Starting   -> Execute
            4:  cur := 5;   // Completing -> Complete
            6:  cur := 1;   // Resetting  -> Idle
            7:  cur := 8;   // Holding    -> Held
            9:  cur := 3;   // Unholding  -> Execute
            10: cur := 11;  // Suspending -> Suspended
            12: cur := 3;   // Unsuspending -> Execute
            13: cur := 14;  // Aborting   -> Aborted
            15: cur := 1;   // Clearing   -> Idle
            16: cur := 17;  // Stopping   -> Stopped
        ELSE
            // stable state — nothing to advance
        END_CASE;
    END_IF;

    State := cur;
    InExecute := (cur = 3);
    IsHeld := (cur = 8);
    IsStopped := (cur = 17);
    IsAborted := (cur = 14);
    Done := TRUE;
END_FUNCTION_BLOCK
