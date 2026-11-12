// t05_rules.st — FB_AtcTuneRules on a known FOPDT model
// (K=2, Tau=20, Theta=5). Rule 0 IMC lambda=max(1.25,4)=4:
//   Kp=(20+2.5)/(2*(4+2.5))=1.7308 Tn=22.5 Tv=100/45=2.2222
// Rule 1 SIMC lambda=5: Kp=20/(2*10)=1.0 Tn=min(20,40)=20 Tv=0
// flags: b0 IMC done, b1..3 IMC values, b4 SIMC done, b5 SIMC Kp,
//        b6 ZN done, b7 ZN Kp = 1.2*20/(2*5) = 2.4
PROGRAM T05
VAR
    flags AT %QB0 : BYTE;
    r0 : FB_AtcTuneRules;
    r1 : FB_AtcTuneRules;
    r2 : FB_AtcTuneRules;
    kp0 AT %QD8 : LREAL;
    kp1 AT %QD16 : LREAL;
END_VAR
    r0(Execute := TRUE, K := 2.0, Tau := 20.0, Theta := 5.0,
       Rule := 0, Lambda := 0.0);
    r1(Execute := TRUE, K := 2.0, Tau := 20.0, Theta := 5.0,
       Rule := 1, Lambda := 0.0);
    r2(Execute := TRUE, K := 2.0, Tau := 20.0, Theta := 5.0,
       Rule := 2, Lambda := 0.0);
    kp0 := r0.Kp;
    kp1 := r1.Kp;
    flags := 0;
    IF r0.Done THEN flags := flags + 1; END_IF;
    IF r0.Kp > 1.70 AND r0.Kp < 1.76 THEN flags := flags + 2; END_IF;
    IF r0.Tn > 22.4 AND r0.Tn < 22.6 THEN flags := flags + 4; END_IF;
    IF r0.Tv > 2.2 AND r0.Tv < 2.3 THEN flags := flags + 8; END_IF;
    IF r1.Done THEN flags := flags + 16; END_IF;
    IF r1.Kp > 0.99 AND r1.Kp < 1.01 THEN flags := flags + 32; END_IF;
    IF r2.Done THEN flags := flags + 64; END_IF;
    IF r2.Kp > 2.39 AND r2.Kp < 2.41 THEN flags := flags + 128; END_IF;
END_PROGRAM
