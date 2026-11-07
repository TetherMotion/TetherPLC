# oracle-beremiz — Beremiz corpus (dev-only test inputs)

`corpus/*.xml` are real PLCopen TC6 project files exported by
Beremiz (github.com/beremiz/beremiz, `exemples/` and `tests/projects/`).
They are consumed as *test inputs* by `plcopen-io`'s import-coverage
test — data, not code, per the oracle quarantine rules (doc 14 §3).
Nothing here is compiled, linked, or shipped.

Coverage target (doc 06 §13.2): ≥95% of FBD constructs (blocks,
wires, vars, comments) must land in the model; gaps surface as
`ImportReport.notes`, never silent drops.
