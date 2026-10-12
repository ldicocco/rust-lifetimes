# Rust lifetimes -- companion files

* `ex/`          every program from the PDF (`ok_*` compile, `fail_*` are rejected on purpose)
* `verify.sh`    compiles them all with `rustc --edition 2021` and checks the expectation
* `lst/`         abridged listings used in the PDF (generated from `ex/`)
* `out/`         real compiler diagnostics / program output quoted in the PDF
* `main.tex`     LaTeX source. Build:  `xelatex -shell-escape main.tex` (run 3x; needs minted + pygments)

Verified with rustc 1.75.0, edition 2021. `fail_06_rpit.rs` is rejected only on edition 2021
(return-position `impl Trait` capture rules changed in edition 2024).
