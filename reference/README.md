# Reference material

This directory keeps the original field guide that the book grew out of. It is
not built, verified or maintained; it is here so that the history of the material
stays available.

- `latex-field-guide/` — the LaTeX source (`main.tex`), its 42 example programs
  (`ex/`), abridged listings (`lst/`), captured diagnostics (`out/`) and its
  verification script. Verified with rustc 1.75.0 and edition 2021.
- `rust-lifetimes-v0.pdf` — the 33-page PDF built from it.
- `rust-lifetimes-v0-examples.zip` — the archive of the same directory that was
  distributed with the PDF.

## License

The same licenses as the rest of the repository apply (see the License section
of the top-level `README.md`). The text, meaning the LaTeX sources (`*.tex`) and
`rust-lifetimes-v0.pdf`, is licensed under CC BY-NC-SA 4.0 (`book/LICENSE`).
The programs (`ex/`), listings (`lst/`), captured diagnostics (`out/`) and
`verify.sh` are available under either the MIT license or the Apache License 2.0,
at your option (`LICENSE-MIT`, `LICENSE-APACHE`). The same split applies to the
files in `rust-lifetimes-v0-examples.zip`.
