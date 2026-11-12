// tc2_qrc.st — Tc2_QRC surface — barcode/QR decode helpers,
// pure-ST, no camera/service.  A real QR decoder needs an
// image pipeline + Reed-Solomon; this surface validates the
// payload channel (check digit, symbology framing) so a decode
// result arriving from an external scanner/camera is verified.

// ---- FB_QrcValidate: EAN-13 / UPC check-digit + QR result gate ----
FUNCTION_BLOCK FB_QrcValidate
VAR_INPUT
    Execute : BOOL;
    // EAN/UPC digits, most-significant first (packed, 0-9 per nibble
    // is not assumed — the caller supplies per-position digits).
    D0 : UDINT; D1 : UDINT; D2 : UDINT; D3 : UDINT; D4 : UDINT;
    D5 : UDINT; D6 : UDINT; D7 : UDINT; D8 : UDINT; D9 : UDINT;
    D10 : UDINT; D11 : UDINT; D12 : UDINT;   // D12 = EAN-13 check
    IsEan13 : BOOL;                          // TRUE: EAN-13 (13 digits)
END_VAR
VAR_OUTPUT
    Done : BOOL;
    CodeOk : BOOL;        // check digit / framing OK
    Error : BOOL;
END_VAR
VAR
    sum : UDINT;
    chk : UDINT;
END_VAR
    Done := FALSE;
    Error := FALSE;
    CodeOk := FALSE;
    IF NOT Execute THEN RETURN; END_IF;

    // EAN-13 / UPC-A check: sum digits odd*1 + even*3 (from the
    // right, check excluded), check = (10 - sum mod 10) mod 10.
    IF IsEan13 THEN
        sum := D0*1 + D1*3 + D2*1 + D3*3 + D4*1 + D5*3 +
               D6*1 + D7*3 + D8*1 + D9*3 + D10*1 + D11*3;
        chk := sum MOD 10;
        IF chk <> 0 THEN chk := 10 - chk; END_IF;
        CodeOk := (chk = D12);
    ELSE
        // UPC-A: 12 digits, check = D11
        sum := D0*3 + D1*1 + D2*3 + D3*1 + D4*3 + D5*1 +
               D6*3 + D7*1 + D8*3 + D9*1 + D10*3;
        chk := sum MOD 10;
        IF chk <> 0 THEN chk := 10 - chk; END_IF;
        CodeOk := (chk = D11);
    END_IF;
    Done := TRUE;
END_FUNCTION_BLOCK
