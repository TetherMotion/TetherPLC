PROGRAM ShActive
VAR
    cnt : DINT;
    out_cnt AT %QD0 : DINT;
END_VAR
cnt := cnt + 1;
out_cnt := 2 * cnt;
END_PROGRAM
