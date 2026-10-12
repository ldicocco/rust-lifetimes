# Rust Lifetimes — Book Outline

**Full title:** *Rust Lifetimes — A Field Guide to Regions, Elision, Variance,
'static and Higher-Ranked Bounds*

**Reader.** A programmer who writes Rust (ownership and borrowing, structs and
enums, traits and generics, closures, iterators, `Result`, Cargo) and has met
lifetime errors they could fix without being able to explain. No knowledge of type
theory or compiler internals is assumed. They want to *understand*: what a
lifetime is, why a rule exists, what a diagnostic means, and which design to choose.

**Versions.** Rust 1.99.0, edition 2024, standard library only. Edition 2021
behavior is shown only where an edition changed the rules (the capture rules of
`impl Trait`, temporary scopes). Nightly-only features are described as such with
the date of the nightly that was checked.

**Origin.** The book grew out of a 33-page LaTeX field guide (rustc 1.75, edition
2021), kept for reference in `reference/latex-field-guide/` with its PDF. Chapters
1–9 rework and extend that material; Chapters 5 and 8 and Part III are new.

**Method.** Every statement about what the compiler accepts or rejects is backed
by a program: an example that compiles (its output is shown) or a program that is
rejected (its diagnostics are shown, captured from rustc 1.99). Diagnostics are
read with the reader, label by label.

**Threads that run through the book.**

- *The signature is the contract.* The body is checked against the signature;
  every caller is checked against the signature and nothing else. This explains
  why elision never looks at the body (Ch. 3), why defaults matter (Ch. 6–8), and
  what a higher-ranked bound promises (Ch. 9).
- *Who picks the lifetime?* A lifetime parameter is picked by the caller; a
  higher-ranked lifetime is picked by the callee, at every use (Ch. 9, 10, 11).
- *What the compiler writes for you.* Elision (Ch. 3), `'_` (Ch. 6), default
  trait-object bounds and `impl Trait` captures (Ch. 8), `Fn` sugar (Ch. 9).
  Every default is shown expanded.
- *Regions, not durations.* A lifetime is a set of program points (Ch. 1); the
  outlives order makes them a subtyping relation (Ch. 4).

**Examples domain.** Text processing (tokenizers, parsers, lookups, config),
small data structures, caches and registries, workers and threads, callbacks.
Never games.

Chapter numbers below are final — refer to other chapters as "Chapter N". Code
files are named after the chapter: `code/examples/chNN_*.rs`,
`code/compile_fail/chNN_*.rs`.

---

## Preface — `00-preface.md`

Who the book is for; how it is verified (pinned compiler, every program compiled,
every diagnostic captured); how to read the listings and the diagnostics; the
versions; how to run the code; the threads above; acknowledgment that it grew
out of the earlier field guide.

## Part I — The Model

### Chapter 1 · Regions, Not Durations — `01-regions.md`

- A lifetime is a *region*: a set of points in the control-flow graph of a
  function body, never a runtime value. Lifetimes are erased before code
  generation.
- Scope of a value vs region of a borrow; the lifetime of a reference is not the
  lifetime of the referent.
- Non-lexical lifetimes: a borrow is live from its creation to its last use (the
  NLL timeline diagram; `ok`/`fail` pair with `v.push`). Shared vs mutable
  borrows: aliasing XOR mutation; E0502, E0499.
- Annotations describe relationships; they never extend anything. `longest` and
  the dangling caller (E0597). The signature is the contract.
- Reading a borrow-checker diagnostic: the labels "borrowed value does not live
  long enough", "dropped here while still borrowed", "borrow later used here".
- Notation: `'a`, `&'a T`, `'a: 'b`, `T: 'a`, `'static`, `'_`, `for<'a>`,
  `use<..>`; implied bounds (`&'a T` implies `T: 'a`).
- Lifetimes in loops and branches (a borrow can be live on one path and dead on
  another).

### Chapter 2 · Whose Borrow Is It? Lifetimes in Types and Methods — `02-whose-borrow.md`

- A struct that stores a reference needs a lifetime parameter, which is part of
  the type (`Parser<'src>`).
- Inside `impl<'src> Parser<'src>`: the data's lifetime vs the anonymous lifetime
  of `&self`/`&mut self`. Returning `Option<&'src str>` vs the elided form that
  pins the parser (E0499).
- Why `slice::Iter<'a, T>::next` can return `&'a T`; implementing `Iterator` for a
  borrowing struct; a lending iterator (items borrowing from `&mut self`) cannot
  be an `Iterator` (pointer to Chapter 10).
- Enums with references; structs with two lifetime parameters and when one is
  enough; lifetime parameters on traits (`trait Source<'a>`).
- The `&'a mut self` trap in `impl<'a> S<'a>`: borrowing a value for its whole
  lifetime ("borrowed forever"); pointer to Chapters 4 and 14.

### Chapter 3 · Lifetime Elision — `03-elision.md`

- Elision is a syntactic rewrite of signatures; it never looks at the body.
- The three rules, stated precisely: rule 1 (fresh lifetimes for inputs); rule 2
  as rustc implements it (exactly one parameter mentions lifetimes, and it has
  exactly one; the Reference's wording differs); rule 3 for `&self`, `&mut self`,
  `self: Pin<&mut Self>` and, since Rust 1.81, receivers whose referent contains
  `Self` such as `self: &Box<Self>`.
- "You write / the compiler reads" table, including fn pointers, `Fn` sugar,
  `impl` headers, `static`/`const` items and associated consts (`'static` in
  every position since 1.99).
- Why the E0106 cases fail; why the compiler refuses to guess; the three honest
  contracts.
- Why a struct field cannot be elided (and why `'static` would be a bad default).
- Candidate extra rules (first parameter / unify everything / infer from body),
  each tested and shown to fail, including the "borrowed forever" consequence of
  unifying lifetimes behind `&mut`.
- The `mismatched_lifetime_syntaxes` lint (1.89) in one paragraph (details in
  Chapter 6).

### Chapter 4 · Subtyping and Variance — `04-variance.md`

- The outlives order on regions; `&'a T <: &'b T` when `'a: 'b`; `'static` is the
  largest region. Diagram.
- Variance: covariant, contravariant, invariant; the table for `&'a T`,
  `&'a mut T`, `Box`, `Vec`, `Cell`, `UnsafeCell`, raw pointers, `fn(T) -> U`,
  `dyn Trait + 'a`, `PhantomData<T>`.
- Covariance in action (shrinking `&'static str`, through `Box`).
- Invariance as the safety net for writes: `&mut &'a str`, `Cell<&'a str>`.
- Contravariance of function arguments, with an example.
- Variance of your own types is computed from their fields; how to read it.
- Invariance in practice: `&'a mut T<'a>` and why it locks a value (pointer to
  Chapter 14); variance in error messages.

### Chapter 5 · Borrows in Practice: Reborrows, Temporaries and the Limits of NLL — `05-borrows-in-practice.md`

- Reborrowing: `&mut *r`, implicit reborrows at call sites, why `let r2 = r` moves
  a `&mut` but `f(r)` does not.
- Two-phase borrows (`v.push(v.len())`).
- Splitting borrows: disjoint fields, `split_at_mut`, why methods cannot split
  through `&mut self`.
- Moves while borrowed (E0505); returning references to locals (E0515).
- Temporaries: temporary lifetime extension (`let r = &make();`), E0716; the
  edition 2024 changes to tail-expression temporaries and `if let` scrutinee
  temporaries.
- Drop and borrows: drop order, the drop check ("borrow might be used here, when
  `x` is dropped"), why `Vec<&T>` is fine but a `Drop` type holding `&T` is
  stricter.
- The limits of NLL: conditional return of a borrow (NLL problem case #3) and
  Polonius (accepted by nightly with Polonius Alpha, checked 2026-10-08; status
  to be re-checked).

## Part II — Lifetimes in Signatures

### Chapter 6 · The Anonymous Lifetime `'_` — `06-anonymous-lifetime.md`

- `'_` is not a name: a fresh lifetime by position (parameter, return type,
  `impl` header, `dyn`/`impl` bound, body type, `use<'_>`); never two equal.
- Job 1: making hidden lifetime parameters visible (`SplitWhitespace<'_>`); the
  warn-by-default `mismatched_lifetime_syntaxes` lint (1.89) and the
  allow-by-default `elided_lifetimes_in_paths` (in `rust_2018_idioms`).
- Job 2: `impl` headers; when you need a real name instead.
- Job 3: letting trait objects borrow (`Box<dyn Fn(&str) -> String + '_>`), with
  a pointer to Chapter 8 for the full rules.
- What `'_` cannot do: it does not resolve ambiguity, it does not mean "infer
  from the body". `'_` in bodies as an inference placeholder.
- Checklist: `'a`, `'_`, or nothing?

### Chapter 7 · The `'static` Lifetime — `07-static.md`

- Two meanings: `&'static T` (a reference) and `T: 'static` (a bound). Diagram of
  the sets.
- Where `&'static` comes from: literals, `static`/`const`, constant promotion,
  `Box::leak`, `OnceLock`/`LazyLock` globals.
- `T: 'static` does not mean "lives forever" (a `Drop` type that is dropped
  immediately); the "does it contain a borrow that can expire?" table.
- Why APIs ask for `'static`: threads (`spawn` vs `thread::scope`, E0373, E0521),
  `Any` and `TypeId`, stored callbacks, generic propagation (E0310).
- The hidden `'static` of `Box<dyn Trait>` in one example (full rules in Chapter
  8).
- Myths and anti-patterns; the decision ladder (own, scope, mix, leak).

### Chapter 8 · Trait Objects, `impl Trait` and Captured Lifetimes — `08-trait-objects-impl-trait.md`

- Default trait-object lifetime bounds: the full rule set with examples
  (`Box<dyn T>`, `&'a dyn T`, `Ref<'a, dyn T>`, traits with lifetime bounds), and
  the caveat that the Reference's description of the precedence is known to be
  inaccurate (what rustc does, verified).
- `Box<dyn Error>` and `?`: errors that borrow their input; `+ '_` vs owning the
  data.
- Unsizing coercions and the object bound (`&'a T` → `&'a (dyn Trait + 'a)`);
  `dyn Trait + 'a` fields in structs.
- Return-position `impl Trait`: what an opaque type captures. Edition 2024
  captures every in-scope generic parameter, including lifetimes; the
  over-capturing failure; precise capturing with `use<..>` (1.82, in traits 1.87).
- `impl Trait + 'a` (an outlives bound) vs `use<'a>` (a capture list).
- Edition 2021 rules and E0700, for code you will still read.
- `impl Trait` in traits (RPITIT) and `async fn` always capture; argument-position
  `impl Trait`.

### Chapter 9 · Higher-Ranked Trait Bounds — `09-higher-ranked.md`

- Who picks the lifetime: caller-chosen parameters vs `for<'a>` (callee-chosen,
  per use). Diagram. Reading `for<'a>` as "for all".
- Where the compiler writes `for<'a>` for you (`Fn` sugar, fn pointers, `dyn Fn`,
  fn items, closures passed to a bound).
- When you must write it: relations the sugar cannot express (two inputs), bounds
  on borrowed types (`for<'a> &'a C: IntoIterator`), traits with a lifetime
  parameter (`for<'de> Deserialize<'de>` and `DeserializeOwned`).
- Higher-ranked types are more general: subtyping of fn pointers, "one type is
  more general than the other", "implementation of `Fn` is not general enough".
- Closures that return references; fixes; unstable closure binders.
- Async callbacks: why `F: Fn(&str) -> Fut` cannot borrow, and `AsyncFn` (1.85).
- Early-bound and late-bound lifetimes (E0794).
- Limits: E0582, E0316, `for<T>` (unstable), no bounds on binders; reading HRTB
  errors.

## Part III — Going Further

### Chapter 10 · Generic Associated Types and Lending Iterators — `10-gats.md`

- The problem: items that borrow from the iterator itself. GATs (stable since 1.65)
  and `type Item<'a> where Self: 'a`.
- Why the compiler requires `where Self: 'a` (the "missing required bound"
  error) and what it means.
- Implementing a lending iterator (`WindowsMut`), consuming it with `while let`,
  and why `for` loops and iterator adapters do not apply.
- Generic code over a GAT: `for<'a> L::Item<'a>: Debug` and the "current
  limitation of the type system" that turns it into `L: 'static`; workarounds
  (bounds in the trait, a helper trait).
- Other uses: borrowing views of collections (`type View<'a>`), and when a
  lifetime parameter on the trait is the simpler design.

### Chapter 11 · Lifetimes in Async Code — `11-async.md`

- `async fn` returns a future that captures every input lifetime; the future
  holds the borrows until it completes.
- Running futures with only the standard library (`block_on` with `Waker::noop`,
  1.85).
- Spawning: why executors require `F: Future + Send + 'static`; a toy `spawn`
  signature to reproduce the errors; `async move` and owning the data.
- Why scoped tasks are hard (futures can be forgotten) while `thread::scope` works.
- Async callbacks: `AsyncFn` and async closures (1.85); recap of Chapter 9.
- Self-referential futures and why `Pin` exists (just enough to understand the
  messages).
- `async fn` in traits and their captures; `Send` bounds on returned futures
  (return type notation is unstable).

### Chapter 12 · Interior Mutability, Guards and Lifetimes — `12-interior-mutability.md`

- `RefCell::borrow` returns `Ref<'_, T>`: the guard's lifetime is a borrow of the
  cell. Why you cannot return `&T` from it (E0515), and `Ref::map`.
- Guards and temporaries: a guard in a `match` scrutinee or `if let` lives longer
  than you expect; what edition 2024 changed.
- `Mutex`/`MutexGuard`, `RwLock`; scoping guards; double-borrow panics vs compile
  errors.
- `Cell` and invariance (recap of Chapter 4); `OnceCell::get_or_init` returning a
  reference tied to `&self`.
- `thread_local!` and `LocalKey::with`: a closure-scoped borrow (a callback that
  cannot leak the reference).

### Chapter 13 · Lifetimes and `unsafe` Code — `13-unsafe.md`

- Raw pointers carry no lifetime; turning them into references creates an
  *unbounded* lifetime; how to bound it with a signature.
- `PhantomData`: unused lifetime parameters (E0392), tying a raw pointer to a
  borrow, choosing variance (`PhantomData<&'a T>`, `PhantomData<fn(T)>`,
  `PhantomData<*mut T>`, `PhantomData<fn(&'a ()) -> &'a ()>`).
- `slice::from_raw_parts` and similar APIs: who promises the lifetime.
- Self-referential structs: why safe Rust rejects them, indices instead of
  references, and how `Pin` relates.
- The drop check and `#[may_dangle]` (unstable `dropck_eyepatch`), at the level a
  user of `Vec` needs.
- Leaking is safe (`mem::forget`, `Box::leak`): why `thread::scope` takes a
  closure.
- Checking `unsafe` code with Miri (nightly; status to be re-checked).

### Chapter 14 · Designing APIs with Lifetimes — `14-api-design.md`

- Own or borrow: parameters (`&str`, `impl AsRef<str>`, `impl Into<String>`),
  return values, and fields.
- `Cow<'a, str>`, `Arc<str>` and `Rc<str>` as alternatives to lifetime
  parameters.
- Zero-copy parsing: a borrowing token type, and an owned version for storage.
- How many lifetime parameters a struct needs; naming lifetimes after what they
  borrow (`'src`, `'buf`).
- The `&'a mut T<'a>` antipattern ("borrowed forever") and how to fix it.
- Callbacks (`impl Fn(&T)` / higher-ranked), iterators returning references,
  builders, errors that borrow vs own.
- A refactoring walkthrough: from "lifetime soup" to a clear design.

## Appendices

- **A · Setting Up and Verifying the Examples** — `A-setup.md`: rustup and the
  pinned toolchain, running examples, `tools/run_examples.py`, reading snapshots,
  nightly for the Polonius examples.
- **B · Error Catalogue** — `B-error-catalogue.md`: every lifetime-related error
  code and message that appears in the book, with its usual meaning, typical fix
  and the chapter that explains it.
- **C · Quick Reference** — `C-quick-reference.md`: "I want to... / write..."
  and the rules of thumb.
- **D · Glossary** — `D-glossary.md`.
- **E · Further Reading** — `E-further-reading.md`: verified links only.

## Possible future chapters

Not part of the current plan, recorded so they are not forgotten: the borrow
checker from the inside (MIR, region inference, Polonius in depth); lifetimes and
FFI (`extern` signatures, callbacks into C); lifetimes in macro-generated code;
case studies of real crates (`serde`'s `'de`, `nom`, `bytes`).
