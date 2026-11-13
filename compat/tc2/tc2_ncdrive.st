// Tc2_NcDrive — NC-axis SoE/CoE drive blocks.
//
// The 20 function blocks of this library are provided by
// compat/tc2/tc2_mc2_drive.st: the NcDrive variants take
// NCTOPLC_AXIS_REF + sNetId where the MC2_Drive variants take
// AXIS_REF + NetId, and both flatten to the same UDINT axis handle
// (slave = Axis - 1) over FB_Soe/FB_MotionAxis.  Covered:
//   FB_SoEReset, FB_SoEWritePassword, FB_SoEExecuteCommand,
//   FB_SoEWriteCommandControl, FB_SoEReadCommandState,
//   FB_SoEReadDiagMessage, FB_SoEReadDiagNumber,
//   FB_SoEReadDiagNumberList, FB_SoEReadClassXDiag, FB_SoERead,
//   FB_SoEWrite, FB_SoEReadAmplifierTemperature,
//   FB_SoEReadMotorTemperature, FB_SoEReadDcBusCurrent,
//   FB_SoEReadDcBusVoltage, FB_CoERead, FB_CoEWrite,
//   FB_SoEAX5000ReadActMainVoltage, FB_SoEAX5000SetMotorCtrlWord,
//   FB_SoEAX5000FirmwareUpdate.
// Deviations (shared with Tc2_MC2_Drive): PVOID buffers flatten to
// scalar Value members, tTimeout is not modeled, bCompleteAccess is
// accepted but sub-index access is unsupported, and the AX5000
// firmware update records the request only.

// F_GetVersionTcNcDrive — library version.
FUNCTION F_GetVersionTcNcDrive : UDINT
F_GetVersionTcNcDrive := 16#0100;
END_FUNCTION
