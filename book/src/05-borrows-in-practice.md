# Borrows in Practice: Reborrows, Temporaries and the Limits of NLL

The first four chapters built a model: a lifetime is a region of the program
(Chapter 1), a type that holds a borrow carries that region as a parameter
(Chapter 2), elision writes the regions of a signature for you (Chapter 3), and
the outlives order makes regions a subtyping relation (Chapter 4). That model
explains why a program is accepted or rejected. It does not yet explain some
things you see every day.

You pass the same `&mut Vec<String>` to a function twice in a row, although a
`&mut` reference cannot be copied. You write `v.push(v.len())`, which borrows `v`
mutably and immutably at once, and the compiler says nothing. You add one method
call to a line, `String::from("x").as_str()`, and get an error about a
"temporary value". You give a struct a `Drop` impl, and code that compiled a
minute ago is rejected with a note about drop order. And now and then the
compiler rejects a function that is plainly correct.

This chapter is about the borrow checker as you meet it in practice: the
mechanics it applies without telling you (reborrows, two-phase borrows), the
values whose lifetime you do not see in the source (temporaries, drops), and the
known cases where it is more conservative than it needs to be.

**In this chapter you will learn:**

- what a reborrow is, when the compiler inserts one, and why `let r2 = r;` moves
  a `&mut` reference while `f(r)` does not;
- how two-phase borrows make `v.push(v.len())` compile, and where they stop;
- how to hold two mutable borrows into one value at once (fields,
  `split_at_mut`, `get_disjoint_mut`), and why accessor methods cannot do it;
- what E0505, E0515 and E0716 mean, and their standard fixes;
- when temporaries are dropped, what lifetime extension does, and what edition
  2024 changed for tail expressions and `if let`;
- how drop order and the drop check decide whether a value with a `Drop` impl
  may hold a borrow;
- which correct programs the current borrow checker still rejects, and what
  Polonius changes.

## Reborrowing

### A `&mut` reference is not `Copy`

A shared reference `&T` is `Copy`: duplicating it creates a second reader, which
is harmless. A mutable reference `&mut T` is not, because its whole point is to
be the *only* path to the data while it is live. Two copies would be two
writers. So assigning a `&mut` to another variable moves it, as with any
non-`Copy` value. The full program is in `code/compile_fail/ch05_move_mut_ref.rs`,
and the compiler rejects it:

```rust
{{#include ../../code/compile_fail/ch05_move_mut_ref.rs:moved}}
```

```text
{{#include ../../code/compile_fail/ch05_move_mut_ref.stderr}}
```

The three labels tell the whole story. The first explains why there is a move at
all: `words` has type `&mut Vec<String>`, "which does not implement the `Copy`
trait". The second marks the move (`let other = words`), and the third the use
after it. Nothing here is about lifetimes; it is ordinary ownership, applied to a
reference.

### Implicit reborrows

Now look at a function that uses its `&mut` parameter twice. It compiles (the
program is `code/examples/ch05_reborrow.rs`):

```rust
{{#include ../../code/examples/ch05_reborrow.rs:implicit}}
```

If `add_word(words, "alpha")` moved `words` into the call, the second call would
be the same error as above. It is not, because the compiler does not pass `words`
itself: it passes a **reborrow**, a new mutable reference to the same place,
written `&mut *words`. You dereference the reference (`*words` is the
`Vec<String>` it points to) and borrow that place again. The new reference has
its own region, which may be shorter than the region of `words` and lasts here
only for the call. While the reborrow is live, `words` is frozen; when it ends,
`words` is usable again.

The compiler inserts a reborrow wherever it knows that the expected type is a
`&mut` reference: an argument for a parameter declared `&mut T`, a `let` with a
written `&mut` type, the receiver of a method that takes `&mut self`. The same
`let` that moved `words` above reborrows it as soon as you write the type:

```rust
{{#include ../../code/examples/ch05_reborrow.rs:annotated}}
```

The three forms (`let other: &mut Vec<String> = words`, `&mut *words` written by
hand, and the implicit `&mut *words` of `words.sort()`) are the same operation.

### Where no reborrow happens

The rule has a consequence that surprises everyone once: when the expected type
is a type parameter, the compiler does not know that it is a reference, so it
does not reborrow. It moves. From `code/compile_fail/ch05_generic_moves.rs`:

```rust
{{#include ../../code/compile_fail/ch05_generic_moves.rs:generic}}
```

```text
{{#include ../../code/compile_fail/ch05_generic_moves.stderr}}
```

`T` is inferred as `&mut Vec<String>`, and the reference is moved into the first
call. This time the compiler suggests the fix itself: "consider creating a fresh
reborrow of `words` here", that is, `passthrough(&mut *words)`, which is what
`ch05_reborrow.rs` does:

```rust
{{#include ../../code/examples/ch05_reborrow.rs:generic}}
```

The same happens when the `&mut` is wrapped in another type. An
`Option<&mut u32>` is not a reference, so passing it moves it
(`code/compile_fail/ch05_option_mut_moves.rs`):

```rust
{{#include ../../code/compile_fail/ch05_option_mut_moves.rs:option}}
```

```text
{{#include ../../code/compile_fail/ch05_option_mut_moves.stderr}}
```

Here the note points in an unhelpful direction: changing `bump` to take a
reference to the `Option` would work, but it changes the function's contract for
every caller. The local fix is to reborrow what is inside the `Option`:
`counter.as_deref_mut()` borrows `counter` for a moment and returns a new
`Option<&mut u32>` that reborrows the same `u32`:

```rust
{{#include ../../code/examples/ch05_reborrow.rs:option}}
```

### A reborrow freezes the original

While a reborrow is live, the reference it came from cannot be used, not even
to read: that would be a second path to the same data
(`code/compile_fail/ch05_reborrow_frozen.rs`):

```rust
{{#include ../../code/compile_fail/ch05_reborrow_frozen.rs:frozen}}
```

```text
{{#include ../../code/compile_fail/ch05_reborrow_frozen.stderr}}
```

Read the first line carefully: the conflict is on `*words`, the vector, not on
the variable `words`. Borrows are tracked per *place*. `other` borrows `*words`
from line 7 until its last use on line 9, and `words.push` (which reborrows
`*words` again for the call) falls inside that region. Swap lines 8 and 9 and the
program compiles, because `other` is dead by the time `words` is used.

This is also how a function that returns a `&mut` works. The result of

```rust
{{#include ../../code/examples/ch05_reborrow.rs:returned}}
```

is a reborrow of `*words` whose region the caller decides, within the region of
the caller's own borrow. While the caller holds the returned `&mut String`, its
vector is frozen. You met this in Chapter 2 as "the result pins the parser":
it is the same mechanism.

The output of the example confirms that every call went through:

```text
{{#include ../../code/output/ch05_reborrow.txt}}
```

## Two-phase borrows

Method calls evaluate the receiver before the arguments. So
`lengths.push(lengths.len())` means: borrow `lengths` mutably for `push`, then
evaluate `lengths.len()`, which borrows it immutably, then call `push`. By the
rules of Chapter 1 that is a shared borrow inside a mutable one, error E0502.
Yet this compiles (`code/examples/ch05_two_phase.rs`):

```rust
{{#include ../../code/examples/ch05_two_phase.rs:accepted}}
```

```text
{{#include ../../code/output/ch05_two_phase.txt}}
```

The rule that accepts it is called **two-phase borrows**. Some mutable borrows
are created in two steps. When the receiver is evaluated, the `&mut` is only
*reserved*; it is *activated* when the call actually starts. Between the two,
the borrow behaves like a shared borrow, so other code may read the value but
not modify it. In `lengths.push(lengths.len())`, the read happens during the
reservation, finishes, and only then is the borrow activated.

The last two lines of the listing show what "finishes" means. In
`text.push_str(&text.to_uppercase())`, the shared borrow `&text` is used by
`to_uppercase`, which returns a new `String`; the argument passed to `push_str`
borrows that new string, not `text`. In `lengths.push(*first)`, the expression
`*first` copies the number out of the vector during the reservation, and `first`
is never used again. In both cases no shared borrow of the receiver is live when
the mutable one is activated.

The rustc development guide lists the three places where two-phase borrows are
created:
<https://rustc-dev-guide.rust-lang.org/borrow-check/two-phase-borrows.html>.
The method-call receiver is the first. The second is the implicit reborrow of a
`&mut` argument, from the previous section:

```rust
{{#include ../../code/examples/ch05_two_phase.rs:reborrowed}}
```

The third is the `&mut` borrow of the left operand of an overloaded compound
assignment operator, such as `+=` on a type that implements `AddAssign`:

```rust
{{#include ../../code/examples/ch05_two_phase.rs:compound}}
```

```rust
{{#include ../../code/examples/ch05_two_phase.rs:compound_use}}
```

### Where two-phase borrows stop

A `&mut` that you write yourself is an ordinary borrow, active immediately.
Spell the same call in function syntax and it is rejected
(`code/compile_fail/ch05_two_phase_explicit.rs`):

```rust
{{#include ../../code/compile_fail/ch05_two_phase_explicit.rs:explicit}}
```

```text
{{#include ../../code/compile_fail/ch05_two_phase_explicit.stderr}}
```

The labels lay out the order of evaluation: the mutable borrow `&mut lengths`
first, then the immutable borrow in `lengths.len()` while it is active, and
"mutable borrow later used by call" to show why the first borrow is still live.

The reservation also does not excuse a shared borrow that is still *in use* when
the call starts. `text.push_str(&text)` would append a string to itself; the
argument is a borrow of `text` that `push_str` reads while it writes to `text`
(`code/compile_fail/ch05_two_phase_shared.rs`):

```rust
{{#include ../../code/compile_fail/ch05_two_phase_shared.rs:shared}}
```

```text
{{#include ../../code/compile_fail/ch05_two_phase_shared.stderr}}
```

"immutable borrow later used by call" is the key label: the shared borrow is
passed *into* the call, so it overlaps the activated mutable borrow. `push_str`
may reallocate the buffer that `&text` points to while reading from it, so this
is a real bug, not a limitation. The fix is to make the argument independent of `text`, as
`to_uppercase()` did above (or `text.clone()`, or `text.repeat(2)` instead of the
whole call).

## Splitting borrows

Aliasing XOR mutation is a rule about *places*. Two mutable borrows may coexist
when they point to places that cannot overlap. The question is always whether
the compiler can *see* that they do not overlap.

### Disjoint fields

Fields of a struct are distinct places, and the borrow checker tracks them
separately. This compiles (`code/examples/ch05_split_borrows.rs`):

```rust
{{#include ../../code/examples/ch05_split_borrows.rs:fields}}
```

`&mut doc.title` borrows the place `(*doc).title`; `&mut doc.words` borrows
`(*doc).words`. Neither is a prefix of the other, so there is no conflict.

### Methods hide the split

Wrap the same two borrows in accessor methods and the program is rejected
(`code/compile_fail/ch05_split_methods.rs`):

```rust
{{#include ../../code/compile_fail/ch05_split_methods.rs:methods}}
```

```text
{{#include ../../code/compile_fail/ch05_split_methods.stderr}}
```

The first mutable borrow is of `*doc`, the whole document. This is the
signature-is-the-contract principle at work. The signature
`fn title_mut(&mut self) -> &mut String` says: "give me exclusive access to all
of `*self`, and I return a reference that may keep that access for as long as you
use it". The caller is checked against the signature only, so it cannot know
that the body touches only `title`. There is no syntax in stable Rust for "this
method borrows only some fields of `self`".

There are three good ways out:

- **Borrow the fields directly** where they are visible (in the same module, or
  if they are public), as `tag_as_draft` did above.
- **One method, several results.** Inside the body the fields are visible, so a
  single method can split the borrow and return all the pieces at once. The
  caller still sees one borrow of `*doc`, and the two results share it:

```rust
{{#include ../../code/examples/ch05_split_borrows.rs:parts}}
```

- **Destructure** the value, which binds one `&mut` reference per field:

```rust
{{#include ../../code/examples/ch05_split_borrows.rs:destructure}}
```

Closures get the same precision for free since edition 2021: a closure captures
the individual places it uses, not the whole variable, so two closures that touch
different fields can both be alive:

```rust
{{#include ../../code/examples/ch05_split_borrows.rs:closures}}
```

### Slices and indices

Indexing is where the compiler stops being able to see disjointness
(`code/compile_fail/ch05_split_index.rs`):

```rust
{{#include ../../code/compile_fail/ch05_split_index.rs:index}}
```

```text
{{#include ../../code/compile_fail/ch05_split_index.stderr}}
```

The error names the place `words[_]`. The `_` is the borrow checker being honest:
it does not track index values, which are run-time data, so every element of a
slice is the same place to it. `words[0]` and `words[1]` might as well be
`words[i]` and `words[j]`, which can be equal.

The standard library offers safe APIs that prove disjointness, either by
construction or by a check at run time:

```rust
{{#include ../../code/examples/ch05_split_borrows.rs:slices}}
```

`split_at_mut(1)` returns two slices, `[0, 1)` and `[1, len)`, that cannot
overlap by construction. (It is implemented with `unsafe` code, which is the
subject of Chapter 13; its signature promises two independent borrows, and the
promise is what you rely on.) `get_disjoint_mut`, stable since Rust 1.86, takes an
array of indices, checks at run time that they are in bounds and pairwise
distinct, and returns an array of `&mut` references or an error. Iterators such as
`iter_mut()` and `chunks_mut()` give out disjoint `&mut` references one at a time.
The output of the example:

```text
{{#include ../../code/output/ch05_split_borrows.txt}}
```

## Moves while borrowed, and borrows that cannot leave

### Moving a borrowed value (E0505)

A move is a conflicting access, like a write: after the move, the old owner no
longer owns the data, and the new owner may modify or free it. So you cannot move
a value while a borrow of it is still going to be used
(`code/compile_fail/ch05_move_while_borrowed.rs`):

```rust
{{#include ../../code/compile_fail/ch05_move_while_borrowed.rs:moved}}
```

```text
{{#include ../../code/compile_fail/ch05_move_while_borrowed.stderr}}
```

The four labels form a timeline: where `line` was declared, where it was borrowed
(`line.split('=')` returns slices of `line`, so `key` borrows it), where it was
moved (into the vector), and where the borrow is used later (the `println!`).
The error exists only because of the last label: delete the `println!` and the
program compiles, because `key` is dead before the move.

The fixes follow from the timeline. Use the borrow before the move, or move after
the last use. Or stop borrowing: give `key` its own copy, which is what
`code/examples/ch05_ownership_fixes.rs` does:

```rust
{{#include ../../code/examples/ch05_ownership_fixes.rs:move_fix}}
```

A third option is to borrow from the new owner (`lines[0].split('=')`) after the
move.

### Returning a reference to a local (E0515)

A function cannot return a borrow of data it owns, because that data is dropped
when the function returns. The compiler has two wordings for this error,
depending on whether you return the reference directly or a value that contains
one; `code/compile_fail/ch05_return_local.rs` shows both (two functions, one
error each):

```rust
{{#include ../../code/compile_fail/ch05_return_local.rs:returned}}
```

```text
{{#include ../../code/compile_fail/ch05_return_local.stderr}}
```

"returns a value referencing data owned by the current function" is the essence
of E0515. Notice what the signature says: by elision rule 2 (Chapter 3), the
result of `first_word(path: &str) -> &str` borrows from `path`. The body tries
to return a borrow of `text` instead. No lifetime annotation can fix that,
because annotations describe what a reference may point to; they never make
anything live longer (Chapter 1). The fix is to change what is returned: an
owned value,

```rust
{{#include ../../code/examples/ch05_ownership_fixes.rs:return_fix}}
```

or a borrow of something the caller owns (take the text as a parameter), or a
`Cow<'_, str>` that is sometimes borrowed and sometimes owned (Chapter 14).

### A borrow of a temporary (E0716)

The third error of this family has a twist: the value that dies is not a variable
at all. From `code/compile_fail/ch05_temporary_dropped.rs`:

```rust
{{#include ../../code/compile_fail/ch05_temporary_dropped.rs:temporary}}
```

```text
{{#include ../../code/compile_fail/ch05_temporary_dropped.stderr:1:15}}
```

"creates a temporary value which is freed while still in use": the `String`
built by `String::from` has no name, and it is freed at the `;` (the label
"temporary value is freed at the end of this statement" points at it), while
`name` still points into it. The help is the standard fix: give the value a name,
so that it lives until the end of the block. To understand why the compiler
created an unnamed value, and how long such values live, we need a section of
their own.

## Temporaries

### What a temporary is

Many expressions need a *place* to work on: `as_str` takes `&self`, so in
`String::from("config.toml").as_str()` the new `String` must be stored somewhere
before it can be borrowed. When the value is not already in a variable, the
compiler stores it in a **temporary**: an unnamed local that it creates for you.
Every temporary is dropped at the end of its **temporary scope**, which is
usually the end of the enclosing statement.

The rules are easiest to see with a value that announces when it is created and
dropped. `code/examples/ch05_temporaries.rs` defines one:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:noisy}}
```

The first experiment is the default rule:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:statement}}
```

```text
{{#include ../../code/output/ch05_temporaries.txt:1:4}}
```

The `Noisy` is created, `len` reads it, and it is dropped at the `;` of the `let`,
before the next statement. `len` is a `usize`, so nothing refers to the dropped
value, and all is well. In `ch05_temporary_dropped.rs`, the result of `as_str()`
was a reference into the temporary: same timing, but now a dangling reference,
and the compiler rejects it.

### Temporary lifetime extension

There is one exception that you use all the time without noticing. When the
initializer of a `let` borrows a temporary directly, the temporary lives until the
end of the enclosing block, exactly as if it had a name:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:extended}}
```

```text
{{#include ../../code/output/ch05_temporaries.txt:5:9}}
```

This is **temporary lifetime extension**. It is a *syntactic* rule: it applies
when the borrow of the temporary appears in an "extending" position of the
initializer. `code/examples/ch05_lifetime_extension.rs` collects the positions
that rustc 1.99 extends:

```rust
{{#include ../../code/examples/ch05_lifetime_extension.rs:extended}}
```

```text
{{#include ../../code/output/ch05_lifetime_extension.txt}}
```

Extension goes through the operand of `&`, the fields of struct, tuple and array
expressions, the tail of a block, the branches of `if` and `match`, and (in rustc
1.99) the arguments of a tuple-struct or enum-variant constructor such as `Some`.
It does not go through a method call or a function call, because the compiler
cannot know what the function does with the reference. That is why
`String::from("config.toml").as_str()` was an error, and why this second function
in `ch05_temporary_dropped.rs` is one too:

```rust
{{#include ../../code/compile_fail/ch05_temporary_dropped.rs:call}}
```

```text
{{#include ../../code/compile_fail/ch05_temporary_dropped.stderr:17:31}}
```

The fix for both is the help's suggestion: a named variable, which
`ch05_ownership_fixes.rs` shows next to an extended temporary:

```rust
{{#include ../../code/examples/ch05_ownership_fixes.rs:temporary_fix}}
```

### Temporaries in `match` and `if let`

A temporary created in the *scrutinee* of a `match` (the expression being
matched) lives until the end of the whole `match`, in every edition:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:match}}
```

```text
{{#include ../../code/output/ch05_temporaries.txt:10:14}}
```

The arm runs while `c` is still alive, although the value being matched, a
`usize`, has nothing to do with it. The reason is that patterns can bind
references into the scrutinee, so the compiler keeps its temporaries alive for
all the arms; it does not check whether a particular `match` needs that.

For values without a destructor this timing is invisible. It becomes visible when
the temporary holds a borrow *and* has a `Drop` impl, because dropping it then
counts as a use of the borrow (more on this in the next section). Here is such a
type, a view of a list of lines (`code/compile_fail/ch05_if_let_2021.rs`):

```rust
{{#include ../../code/compile_fail/ch05_if_let_2021.rs:lines}}
```

and a function that matches on its first line, and adds a line when there is
none. The compiler rejects it, in every edition
(`code/compile_fail/ch05_match_scrutinee.rs`):

```rust
{{#include ../../code/compile_fail/ch05_match_scrutinee.rs:match}}
```

```text
{{#include ../../code/compile_fail/ch05_match_scrutinee.stderr}}
```

The diagnostic now has a two-part story, linked by "...". First: "a temporary
with access to the immutable borrow is created here ...", pointing at
`Lines::new(lines)`. Then, at the closing brace of the `match`: "... and the
immutable borrow might be used here, when that temporary is dropped and runs the
`Drop` code for type `Lines`". In between, the `None` arm wants to push to the
vector. The `Option<&str>` returned by `first()` is not the problem (in the `None`
arm there is no `&str`); the temporary `Lines` is. The fix is to end the
temporary's life before the `match`, with a `let`
(`code/examples/ch05_temporaries_2024.rs`):

```rust
{{#include ../../code/examples/ch05_temporaries_2024.rs:match_fix}}
```

`if let` used to follow the same rule as `match`, and edition 2024 changed it.
Here is the experiment with `Noisy`, once with a match that succeeds and once with
one that fails:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:if_let}}
```

```text
{{#include ../../code/output/ch05_temporaries.txt:15:21}}
```

The scrutinee's temporaries live through the then-branch (`dog` is dropped after
it) but are dropped *before* the else-branch runs (`cat` is dropped first). In
edition 2021 they lived until the end of the whole `if let`, `else` included. The
`Lines` version of `show_first` written with `if let` shows the difference.
Compiled with edition 2021, it is rejected with the same story as the `match`:

```rust
{{#include ../../code/compile_fail/ch05_if_let_2021.rs:if_let}}
```

```text
{{#include ../../code/compile_fail/ch05_if_let_2021.stderr}}
```

The same function, unchanged, compiles in edition 2024; it is part of
`ch05_temporaries_2024.rs`.

> [!NOTE]
> Edition 2024 (Rust 1.85 and later) drops the temporaries of an `if let`
> scrutinee before the `else` branch. The edition guide describes the change at
> <https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html>.
> The lint `if_let_rescope` (in the `rust_2024_compatibility` group) flags the
> `if let` expressions whose behavior changes when you migrate a crate. A `match`
> keeps the old scope in every edition, and so does `while let` (Chapter 12 shows
> what that means for lock guards).

### Temporaries in tail expressions

The second change of edition 2024 concerns the *tail expression* of a block, the
final expression without a `;` that gives the block its value. In edition 2021,
temporaries created there were dropped *after* the block's local variables, so a
temporary that borrowed a local outlived it. Edition 2024 drops them *before* the
locals:

```rust
{{#include ../../code/examples/ch05_temporaries.rs:tail}}
```

```text
{{#include ../../code/output/ch05_temporaries.txt:22:27}}
```

`tail` is dropped before `_local`, although `_local` was created first. Edition
2021 printed the two drops in the opposite order. When the temporary borrows the
local and has a destructor, the old order is a compile error. Here is a reading
view with a `Drop` impl (`code/compile_fail/ch05_tail_expr_2021.rs`):

```rust
{{#include ../../code/compile_fail/ch05_tail_expr_2021.rs:reader}}
```

and a function whose tail expression creates one. Compiled with edition 2021, it
is rejected:

```rust
{{#include ../../code/compile_fail/ch05_tail_expr_2021.rs:tail}}
```

```text
{{#include ../../code/compile_fail/ch05_tail_expr_2021.stderr}}
```

The labels at the closing brace show the problem in edition 2021's order: first
"`text` dropped here while still borrowed", then "... and the borrow might be used
here, when that temporary is dropped". The note and the help describe the
workaround that edition 2021 code is full of: store the result in a local and
make that local the tail expression. In edition 2024 the function compiles as
written (it is in `ch05_temporaries_2024.rs`), because the `Reader` is dropped
first. The change is described at
<https://doc.rust-lang.org/edition-guide/rust-2024/temporary-tail-expr-scope.html>,
and the migration lint is `tail_expr_drop_order`.

Here are the rules of this section in one table:

| A temporary created in... | is dropped (edition 2024) | in edition 2021 |
|---|---|---|
| a statement, or a `let` without extension | at the end of the statement | same |
| a `let` initializer, extended | at the end of the block | same |
| a `match` scrutinee | at the end of the `match` | same |
| a `while let` scrutinee | at the end of each iteration | same |
| an `if let` scrutinee | before the `else` branch | at the end of the `if let` |
| the tail expression of a block | before the block's locals | after the block's locals |

## Drop order and the drop check

### The order of drops

Rust drops values at precise, predictable points. Local variables are dropped at
the end of their scope, in the *reverse* order of their declaration. The fields of
a struct are dropped in the order they are *declared*, after the struct's own
`Drop` impl (if any) has run. A `let _ = expr;` does not bind anything: `_` is a
pattern that matches without moving, so the value of `expr` stays a temporary and
is dropped at the end of the statement. `code/examples/ch05_drop_order.rs` shows
all three:

```rust
{{#include ../../code/examples/ch05_drop_order.rs:pair}}
```

```rust
{{#include ../../code/examples/ch05_drop_order.rs:order}}
```

```text
{{#include ../../code/output/ch05_drop_order.txt:1:7}}
```

### Dropping is a use

A destructor is code that runs at the end of a scope, and it can read the value's
fields, including the references inside them. So when a value of a type with a
`Drop` impl is dropped, the borrow checker treats the drop as a *use* of every
borrow the value holds. This is the **drop check**. Here is a report that prints a
summary when it goes away:

```rust
{{#include ../../code/compile_fail/ch05_drop_check.rs:report}}
```

Declare the report first and the text second, and the program is rejected
(`code/compile_fail/ch05_drop_check.rs`):

```rust
{{#include ../../code/compile_fail/ch05_drop_check.rs:caller}}
```

```text
{{#include ../../code/compile_fail/ch05_drop_check.stderr}}
```

All the action is at the closing brace. Variables are dropped in reverse order,
so `text` is dropped first ("`text` dropped here while still borrowed"); then
`report` is dropped, and "borrow might be used here, when `report` is dropped and
runs the `Drop` code for type `Report`". The note states the rule that decides
it: "values in a scope are dropped in the opposite order they are defined". If
the compiler accepted this program, `Report::drop` could read the `&str`s in
`self.lines`, which point into a freed `String`. This destructor happens to look
only at the length, but the borrow checker does not read destructors: it goes by
the existence of the `impl Drop`, as it goes by signatures everywhere else.

The fix is the order of declaration. Declare the data before the value that
borrows it, so that the borrower is dropped first:

```rust
{{#include ../../code/examples/ch05_drop_order.rs:report_ok}}
```

```text
{{#include ../../code/output/ch05_drop_order.txt:8:9}}
```

### Why `Vec<&str>` is more relaxed

The same code with a plain `Vec<&str>` instead of a `Report` compiles in the
"wrong" order:

```rust
{{#include ../../code/examples/ch05_drop_order.rs:plain_vec}}
```

```text
{{#include ../../code/output/ch05_drop_order.txt:10:11}}
```

`Vec` has a destructor too (it frees its buffer), so why is its drop not a use of
the `&str`s? Because the standard library tells the compiler that `Vec`'s
destructor does not read the borrowed data of its elements. It only drops the
elements, and dropping a `&str` does nothing. The promise is made with an
unstable attribute, `#[may_dangle]`, that only the standard library can use on
stable Rust; Chapter 13 explains it at the level a user of `Vec` needs. `Box`
makes the same promise, and `Option` has no destructor of its own.

A struct of your own is just as relaxed as long as it has no `Drop` impl,
because then its drop glue only drops its fields one by one:

```rust
{{#include ../../code/examples/ch05_drop_order.rs:notes}}
```

```rust
{{#include ../../code/examples/ch05_drop_order.rs:no_drop}}
```

```text
{{#include ../../code/output/ch05_drop_order.txt:12:13}}
```

So adding an `impl Drop` to a type that holds references is a change to its
public contract: it makes every user's declaration order matter. If the
destructor does not need the borrowed data, consider whether the type needs a
destructor at all, or whether the destructor can live in a field that holds only
owned data.

## The limits of NLL

The borrow checker that you use is called NLL, for non-lexical lifetimes
(Chapter 1). It computes, for each borrow, a region: the set of program points
where the borrow may still be used. It is precise enough that almost every
correct program you write is accepted. Almost.

### NLL problem case #3

Here is a function that looks up a word in a cache, and computes and inserts the
value when it is missing (`code/compile_fail/ch05_nll_case3.rs`):

```rust
{{#include ../../code/compile_fail/ch05_nll_case3.rs:cached}}
```

The function is correct. If the lookup succeeds, it returns the borrowed value and
never touches the map again. If it fails, there is no `found`, so nothing refers
into the map when `insert` modifies it. Stable Rust rejects it anyway:

```text
{{#include ../../code/compile_fail/ch05_nll_case3.stderr}}
```

Read the labels in order. "lifetime `'m` defined here": the caller picks `'m`, and
it covers the whole call, from the caller's point of view. "immutable borrow
occurs here": `cache.get(word)` borrows the map. "returning this value requires
that `*cache` is borrowed for `'m`": because `found` is returned, the borrow
must last for all of `'m`. And then "mutable borrow occurs here" at the `insert`.

The problem is in the third label. NLL represents the borrow by one region, and
the constraint "this region outlives `'m`" does not depend on the path taken: a
region that must outlive a lifetime parameter of the function contains every
point of the function body. So the borrow is considered live at the `insert`
too, although `insert` can only be reached on the path where nothing was
returned:

![NLL problem case #3. The loan created by `cache.get` must last for all of `'m` because it is returned on the `Some` path; NLL therefore considers it live on the `None` path too, at the `insert`. Polonius sees that the loan cannot reach the `insert`.](../images/ch05-nll-case3.svg)

This example has a name. The RFC that introduced NLL listed it as "Problem case
#3: conditional control flow across functions", one of the cases that NLL would
*not* solve:
<https://rust-lang.github.io/rfcs/2094-nll.html>.

### Polonius

**Polonius** is a reformulation of the borrow checker designed to accept such
programs. Roughly, instead of asking at each point "is this region live here?",
it asks "which loans can reach this point, along the paths that actually lead
here?". At
the `insert`, the only path that arrives there is the `None` path, on which the
loan from `cache.get` is not held by any live reference, so there is no conflict.

> [!NOTE]
> As of this writing Polonius is not part of stable Rust. Its first version,
> called Polonius Alpha, is enabled by default on nightly compilers since August
> 2026 (announced at
> <https://blog.rust-lang.org/2026/08/04/enabling-polonius-alpha-on-nightly/>,
> which says the team is "aiming to stabilize prior to the end of the year").
> The nightly of 2026-10-08 (`rustc 1.101.0-nightly`) compiles
> `ch05_nll_case3.rs` without errors, and the flag `-Zpolonius=off` brings the
> E0502 back. Check the current status before you rely on it.

You can try it yourself, from the `code/` directory:

```sh
rustup toolchain install nightly
rustc +nightly --edition 2024 --crate-type lib --emit=metadata \
    --out-dir target compile_fail/ch05_nll_case3.rs
```

### Workarounds on stable Rust

Until then, you rewrite the function so that no borrow is held on the path that
mutates. The entry API does the lookup and the insertion with one mutable borrow
(`code/examples/ch05_nll_workarounds.rs`):

```rust
{{#include ../../code/examples/ch05_nll_workarounds.rs:entry}}
```

It costs a `String` allocation for the key even when the word is already there.
The other classic rewrite asks a question whose answer is a `bool`, so that no
borrow survives the test, and looks up the value a second time:

```rust
{{#include ../../code/examples/ch05_nll_workarounds.rs:lookup_twice}}
```

```text
{{#include ../../code/output/ch05_nll_workarounds.txt}}
```

The recognizable shape of the problem is a function that receives a `&mut`,
returns a borrow obtained from it on one path, and uses it mutably on another
path. If an error carries the label "returning this value requires that ... is
borrowed for `'a`", and the conflicting access is on a path where that value was
never returned, you are looking at case #3. The usual tools are the ones above:
an API that does both steps at once (`entry`), a test that returns no borrow
(`contains_key`, a `bool`, an index), or returning an index or an owned value
instead of a reference.

## Common confusions

**"A `&mut` parameter can be used only once."** It can be used as often as you
like, as long as each use is a reborrow: calls to functions that take `&mut T`,
method calls, and `let` with a written type all reborrow. Only a plain move
(`let r2 = r;`, a generic parameter, a wrapper such as `Option<&mut T>`) gives the
reference away.

**"Two-phase borrows change the order of evaluation."** They do not. The receiver
is still evaluated first; only the *activation* of its mutable borrow waits for
the call. That is why the arguments may read the receiver but not keep a borrow
of it into the call.

**"Lifetime extension keeps temporaries alive when I need it."** Only in the
syntactic positions listed above. A temporary passed through a function or a
method call is never extended, whatever the function returns.

**"I'll call `drop` explicitly to fix the drop order."** For the drop check, the
order of *declaration* matters, and an explicit `drop` does not change it. This
program (`code/compile_fail/ch05_drop_check_explicit.rs`) is still rejected:

```rust
{{#include ../../code/compile_fail/ch05_drop_check_explicit.rs:caller}}
```

```text
{{#include ../../code/compile_fail/ch05_drop_check_explicit.stderr}}
```

And rightly so: if `extend` panicked, unwinding would drop the variables in
reverse order of declaration, `text` and then `report`, whose destructor could
read the freed lines. Declare `text` first.

**"The compiler rejected it, so it must be unsafe."** Usually, yes. But the
borrow checker is conservative by design, and case #3 is a correct program that
stable Rust rejects. When you are sure a program is correct, look for the shape of
case #3 before you look for a bug.

## Summary

- A `&mut T` is not `Copy`. Where the expected type is known to be `&mut`
  (function arguments, `let` with a type, method receivers), the compiler
  *reborrows* it (`&mut *r`) instead of moving it; with generics and wrappers such
  as `Option<&mut T>` it moves, and you reborrow by hand (`&mut *r`,
  `as_deref_mut()`).
- While a reborrow is live, the original reference is frozen. Borrows are tracked
  per place (`*words`, `doc.title`, `words[_]`).
- Two-phase borrows reserve the `&mut` of a method receiver, an implicit argument
  reborrow, or an overloaded compound assignment, and activate it when the call
  starts: reads in between are allowed, a shared borrow passed into the call is
  not. A `&mut` you write yourself is never two-phase.
- Disjoint fields can be borrowed mutably at the same time; accessor methods
  cannot express that, because the signature borrows all of `self`. Return
  several borrows from one method, or destructure. For slices, use
  `split_at_mut`, `get_disjoint_mut` (1.86) or `iter_mut`.
- E0505: a value is moved while a borrow of it is still used. E0515: a function
  returns a borrow of its own local. E0716: a borrow outlives the temporary it
  points to.
- Temporaries die at the end of their statement unless lifetime extension applies
  (a borrow in an extending position of a `let`). `match` and `while let`
  scrutinees keep their temporaries to the end; in edition 2024, `if let` drops
  them before `else`, and tail expressions drop them before the block's locals.
- Locals are dropped in reverse order of declaration, fields in declaration order.
  The drop check treats dropping a value whose type has a `Drop` impl as a use of
  its borrows; `Vec<&T>` and types without a `Drop` impl are exempt.
- NLL rejects some correct programs, most famously a borrow returned on one path
  and a mutation on another (problem case #3). Polonius, on by default on nightly
  since August 2026, accepts it; on stable, use the entry API or a test that holds
  no borrow.

## Exercises

1. Without compiling, predict which of these calls compile when `buf` has type
   `&mut String`: `fill(buf); fill(buf);` with `fn fill(b: &mut String)`;
   `show(buf); show(buf);` with `fn show<T: std::fmt::Debug>(x: T)`;
   `let b2 = buf; fill(b2); fill(buf);`. Then check your answers and fix the
   ones that fail with the smallest change.
   *Hint:* ask, at each call, whether the compiler knows that the expected type is
   a `&mut` reference.

2. Give `Document` from `ch05_split_borrows.rs` a third field, `errors:
   Vec<String>`, and write a method that returns mutable references to `words`
   and `errors` at the same time. Then write a function that moves every word that
   contains a digit from `words` to `errors`.
   *Hint:* the method's body can borrow the two fields separately; `retain` and
   `push` on the two results do the rest.

3. Copy `ch05_temporaries.rs` to a scratch directory and compile it with
   `rustc --edition 2021`. Which lines of the output change, and why? Then delete
   the `impl Drop for Lines` from `ch05_match_scrutinee.rs` and compile it again:
   what happens, and what does that tell you about when temporary scopes matter?
   *Hint:* without a destructor, dropping the temporary is not a use of the
   borrow it holds.

4. Make `ch05_drop_check.rs` compile in three different ways: by changing the
   order of declarations, by changing the field type so that `Report` owns its
   lines, and by moving the printing out of the `Drop` impl. Which one would you
   choose for a type in a library, and why?
   *Hint:* think about what each change costs the *users* of `Report`.

5. Write `fn longest_or_insert<'v>(words: &'v mut Vec<String>) -> &'v String`,
   which returns the longest word if it has more than five letters and otherwise
   pushes `"(none)"` and returns a reference to it. Write it first in the natural
   way, observe the error, then write a version that compiles on stable Rust.
   *Hint:* find the *index* of the longest word first; an index holds no borrow.
