# Interior Mutability, Guards and Lifetimes

The borrow checker enforces one rule above all: at any point, a value has either
any number of readers or exactly one writer. Most of the time that rule costs you
nothing. Sometimes it is in the way. A cache wants to fill itself in a method that
takes `&self`. A command history is appended to by an editor and by an autosave
task that both hold a shared reference to it. A counter in a struct is bumped by
every read. Several threads push results into one vector.

Rust's answer is **interior mutability**: types such as `Cell`, `RefCell`,
`OnceCell`, `Mutex` and `RwLock` allow mutation through a shared reference, and
keep it safe by other means: a check at run time, a lock, or an API that never
hands out a reference that a mutation could invalidate. It is tempting to think
that these types switch lifetimes off. They do not. Every reference into a cell
is still a borrow with a region, and most of these types hand out **guards**,
values that carry a lifetime parameter and run code when they are dropped. That
is exactly the kind of value whose lifetime Chapter 5 showed to be subtle.

This chapter is about the place where the compile-time checks of this book meet
the run-time checks of interior mutability.

**In this chapter you will learn:**

- what `RefCell::borrow` returns: a guard `Ref<'_, T>` that is a compile-time
  borrow of the cell and a run-time borrow of its contents;
- why a method cannot return `&T` from inside a `RefCell` (E0515), and three ways
  out: `Ref::map`, a closure, an owned copy;
- why a guard in a `match`, a `while let` or an extended `let` lives longer than
  you expect, and what edition 2024 changed;
- how `Mutex` and `RwLock` guards follow the same rules, and how to keep the
  locked region short;
- which conflicts are compile errors and which are panics or deadlocks;
- why `Cell` needs no guard, why it is invariant, and why
  `OnceCell::get_or_init` can return a plain `&T`;
- how `thread_local!` and `LocalKey::with` lend a reference to a closure that
  cannot leak it.

## Two kinds of checking

`&T` and `&mut T` encode the readers-or-writer rule in types, and the borrow
checker verifies it before the program runs. Interior mutability keeps the rule
but enforces it in a different way. At the bottom of every such type is
`UnsafeCell<T>`, the one type through which the language allows mutation behind a
shared reference; each safe wrapper adds its own discipline on top:

| Type | Mutates through `&self` with | Keeps aliasing safe by | Hands out |
|---|---|---|---|
| `Cell<T>` | `set`, `replace`, `take` | never lending a reference to the contents | copies and moved values |
| `RefCell<T>` | `borrow_mut` | counting borrows at run time | `Ref`, `RefMut` guards |
| `OnceCell<T>` | `get_or_init`, once | never changing the contents after that | `&T` |
| `Mutex<T>` | `lock` | a lock that blocks other threads | `MutexGuard` |
| `RwLock<T>` | `write` | a readers-writer lock | read and write guards |

The first three are for a single thread; `Mutex` and `RwLock` are their
thread-safe counterparts (as `OnceLock` is for `OnceCell`, Chapter 7). The rest of
the chapter takes them in the order that matters for lifetimes: first the guards,
then the types that need none.

## `RefCell` and its guards

### Mutation through `&self`

Here is a command history that can be appended to through a shared reference.
The full program is `code/examples/ch12_refcell_guard.rs`:

```rust
{{#include ../../code/examples/ch12_refcell_guard.rs:history}}
```

`record` takes `&self`, yet it pushes to the vector. Two parts of a program can
now hold `&History` at the same time and both record entries:

```rust
{{#include ../../code/examples/ch12_refcell_guard.rs:shared}}
```

```text
{{#include ../../code/output/ch12_refcell_guard.txt:1:1}}
```

### What `borrow` returns

The signatures explain everything else. Simplified from the standard library's
source (Rust 1.99; attributes, `const` and the `?Sized` bounds omitted), they read:

```rust,ignore
impl<T> RefCell<T> {
    pub fn borrow(&self) -> Ref<'_, T>;
    pub fn borrow_mut(&self) -> RefMut<'_, T>;
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError>;
}

pub struct Ref<'b, T> { /* a pointer to the value and to the borrow count */ }
```

By elision (Chapter 3), the `'_` in the result is the lifetime of `&self`: the
guard *borrows the cell*. That gives every guard two layers of checking.

**At compile time**, a `Ref<'b, T>` holds an ordinary shared borrow of the cell
for `'b`. Everything this book has said about borrows applies to it: the cell
must outlive the guard, it cannot be moved while a guard exists, and so on. This
program, `code/compile_fail/ch12_guard_outlives_cell.rs`, is rejected exactly as
if `entries` were a `&Vec<String>`:

```rust
{{#include ../../code/compile_fail/ch12_guard_outlives_cell.rs:outlives}}
```

```text
{{#include ../../code/compile_fail/ch12_guard_outlives_cell.stderr}}
```

**At run time**, the cell counts its guards. `borrow` succeeds when there is no
`RefMut`, and adds one reader; `borrow_mut` succeeds only when there is no guard
at all; and each guard's destructor gives its borrow back. The compiler cannot do
this part, because `borrow` and `borrow_mut` both take `&self`: to the borrow
checker, two calls are two shared borrows of the cell, which never conflict.

![The run-time borrow state of a `RefCell`. Guards move the cell between states; a conflicting request panics, or returns an error with the `try_` methods.](../images/ch12-refcell-states.svg)

`try_borrow_mut` asks for a `RefMut` without panicking, which makes the run-time
state visible:

```rust
{{#include ../../code/examples/ch12_refcell_guard.rs:dynamic}}
```

```text
{{#include ../../code/output/ch12_refcell_guard.txt:2:4}}
```

The `Ref` named `entries` keeps the cell in the "reading" state until
`drop(entries)`; the next request succeeds. For a guard, the end of its *region*
in the borrow checker's sense (the last use) and the end of the run-time borrow
(the drop) are different points. A guard that is no longer used but not yet
dropped still blocks writers. With guards, *when a value is dropped* matters far
more than *when it was last used*.

### Compile error or panic?

Since the run-time layer is invisible to the compiler, a conflict between two
guards compiles and panics when it happens. `code/examples/ch12_double_borrow.rs`
provokes both conflicts and catches the panics to print their messages:

```rust
{{#include ../../code/examples/ch12_double_borrow.rs:panics}}
```

```text
{{#include ../../code/output/ch12_double_borrow.txt}}
```

"RefCell already borrowed" means that `borrow_mut` found a guard; "RefCell
already mutably borrowed" means that `borrow` found a `RefMut`. The last line
shows that unwinding dropped the guards, so the cell is usable again afterwards.

The rule of thumb is simple. A conflict between two *guards* of the same cell is
found at run time. A conflict between a guard and something that needs the cell
*itself* (moving it, dropping it, calling a `&mut self` method such as
`RefCell::get_mut`) is found at compile time, because the guard is an ordinary
borrow of the cell.

### Returning a reference from inside a `RefCell` (E0515)

The most common surprise with `RefCell` is a getter that will not compile. A
method on `History` that returns the last entry
(`code/compile_fail/ch12_return_ref_from_guard.rs`):

```rust
{{#include ../../code/compile_fail/ch12_return_ref_from_guard.rs:last}}
```

```text
{{#include ../../code/compile_fail/ch12_return_ref_from_guard.stderr}}
```

"temporary value created here" points at `self.entries.borrow()`: the guard is a
temporary of the method's body. The `&str` is obtained *through* it, by
`Deref::deref(&self) -> &Self::Target`, so its lifetime is that of a borrow of
the guard, a value that dies when the method returns. Hence "returns a value
referencing data owned by the current function", the E0515 of Chapter 5.

It helps to see why this must be so. If you could get a `&'b str` with the
lifetime of the guard's *type* instead, you could drop the guard (resetting the
count), keep the `&str`, and then call `borrow_mut` and clear the vector, leaving
the `&str` dangling. The run-time count only works if every reference into the
cell is tied to a live guard, and the signature of `deref` makes sure it is.

### Three ways out

There are three honest signatures for "give the caller access to data inside a
`RefCell`". `code/examples/ch12_ref_map.rs` implements all three. The first
returns a guard, narrowed to the part the caller asked for:

```rust
{{#include ../../code/examples/ch12_ref_map.rs:guards}}
```

`Ref::map` takes a guard and a function that projects the borrowed value to a
part of it, and returns a guard for that part; the run-time borrow is
transferred, not released. `Ref::filter_map` does the same for a projection that
may fail, and gives the original guard back in the `Err` case (stable since Rust
1.63). Both are associated functions, called as `Ref::map(guard, f)` rather than
`guard.map(f)`: as the standard library's documentation puts it, "A method would
interfere with methods of the same name on the contents of a `RefCell` used
through `Deref`."

The return type `Option<Ref<'_, str>>` is honest: it says that the caller holds a
run-time borrow of the history until it drops the result. The second design does
not let the caller hold anything; it lends the data to a closure. The third gives
up on borrowing and returns an owned copy:

```rust
{{#include ../../code/examples/ch12_ref_map.rs:closure}}
```

In `with_last`, the closure's parameter is `Option<&str>` with an elided
lifetime, so the bound is higher-ranked (Chapter 9): `f` must work for a borrow of
*any* lifetime, including one that ends when `with_last` returns. Its result type
`R` is chosen by the caller before the call and cannot mention that lifetime, so
the `&str` cannot escape. We will meet the same shape again with `thread_local!`.
Here is how the three look from the caller's side:

```rust
{{#include ../../code/examples/ch12_ref_map.rs:use}}
```

```text
{{#include ../../code/output/ch12_ref_map.txt}}
```

The comment after the `if let` is the important one: the guard returned by
`last()` is released at the end of the `if let`, so the `borrow_mut` on the next
line succeeds. Had the caller kept `last` in a variable until the end of the
function, that `borrow_mut` would have panicked. A guard in a return type moves
the responsibility for the run-time borrow to the caller; a closure or an owned
value keeps it inside the method.

## Guards and temporaries

A guard you create inside an expression and do not bind to a variable is a
temporary, and the temporary scope rules of Chapter 5 decide when it is dropped.
For most temporaries that timing is invisible. For a guard it decides whether
the next `borrow_mut` succeeds. `code/examples/ch12_guard_temporaries.rs` uses a
small helper to report the state of a cell without panicking:

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:status}}
```

### `match`: alive for every arm

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:match}}
```

```text
{{#include ../../code/output/ch12_guard_temporaries.txt:1:1}}
```

The scrutinee is `words.borrow().len()`, a plain `usize`, yet the arm runs with
the cell borrowed: the guard is a temporary of the scrutinee, and those live until
the end of the `match`, in every edition. Call `words.borrow_mut()` in that arm
and the program panics. The fix is to compute the value first, with a `let`:

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:let_first}}
```

```text
{{#include ../../code/output/ch12_guard_temporaries.txt:4:4}}
```

### `if let`: released before `else` (edition 2024)

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:if_let}}
```

```text
{{#include ../../code/output/ch12_guard_temporaries.txt:2:3}}
```

The guard lives through the then-branch, and is released before the else-branch.
That second half is new in edition 2024. In edition 2021 the guard lived until the
end of the whole `if let`, and a common pattern, "look the key up; if it is
missing, insert it in the `else` branch", panicked with "RefCell already
borrowed". Chapter 5 shows the edition 2021 rule with a program that the compiler
rejects (`ch05_if_let_2021.rs`); with a `RefCell`, the same mistake compiles and
fails at run time, which is worse.

### Extended `let`: alive until the end of the block

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:extended}}
```

```text
{{#include ../../code/output/ch12_guard_temporaries.txt:5:5}}
```

This one is easy to miss. `&words.borrow()[0]` borrows an element of the vector
*through* the guard, and the borrow appears in an extending position of the `let`
(the operand of `&`, Chapter 5). So the guard is lifetime-extended to the end of
the block, as if you had written `let guard = words.borrow();`. Without the
extension, `first` would point into a guard that was already dropped. Either way,
a reference into a `RefCell` keeps a guard alive, visibly or not.

### `while let`: alive for each iteration

```rust
{{#include ../../code/examples/ch12_guard_temporaries.rs:while_let}}
```

```text
{{#include ../../code/output/ch12_guard_temporaries.txt:6:7}}
```

The `RefMut` created by `words.borrow_mut()` lives through the whole body of each
iteration, not only through the `pop()`. A work queue whose loop body pushes
follow-up jobs (`words.borrow_mut().push(...)`) panics on the first iteration. The
fix has the same shape as for `match`: pop in a statement of its own, then look at
the result (we will write it out for `Mutex` below).

### Tail expressions (edition 2024)

The second edition 2024 change of Chapter 5 matters for guards too. A function
whose last expression reads a local `RefCell` creates the guard in its tail
expression. In edition 2021, that guard was dropped after the local it borrows,
and the function did not compile (`code/compile_fail/ch12_tail_guard_2021.rs`,
compiled with `edition=2021`):

```rust
{{#include ../../code/compile_fail/ch12_tail_guard_2021.rs:tail}}
```

```text
{{#include ../../code/compile_fail/ch12_tail_guard_2021.stderr}}
```

The story is the one Chapter 5 read label by label: a temporary with access to the
borrow is created in the tail expression, `history` is dropped at the closing
brace, and then the borrow "might be used here, when that temporary is dropped and
runs the destructor for type `Ref<'_, Vec<String>>`". (Chapter 5's `Reader` had
"the `Drop` code for type"; `Ref` has no `Drop` impl of its own, but one of its
fields has, so the compiler speaks of its *destructor*.) In edition 2024 the guard
is dropped before the locals, and the function compiles unchanged: it is
`entry_count` in `ch12_guard_temporaries.rs`, whose last line of output is
`entry_count() = 1`.

Here is the whole section in one table:

| You write | The guard is released |
|---|---|
| `let n = cell.borrow().len();` | at the `;` |
| `let guard = cell.borrow();` | when `guard` goes out of scope, or at `drop(guard)` |
| `let first = &cell.borrow()[0];` | at the end of the block (extended) |
| `match cell.borrow().len() { .. }` | after the `match` |
| `if let .. = cell.borrow().. { } else { }` | before `else` (2024), after it (2021) |
| `while let .. = cell.borrow_mut().pop() { }` | at the end of each iteration |
| `cell.borrow().len()` as a tail expression | before the locals (2024), after them (2021) |

## `Mutex`, `RwLock` and their guards

### A `MutexGuard` is a borrow of the mutex

`Mutex<T>` is the thread-safe relative of `RefCell<T>`, and its signature has the
same shape (simplified the same way):

```rust,ignore
impl<T> Mutex<T> {
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>;
    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>>;
}
```

The guard borrows the mutex for `'_`, gives access to the `T` through `Deref` and
`DerefMut`, and unlocks the mutex in its destructor. (`LockResult` is a `Result`
because of *poisoning*: if a thread panics while holding the guard, later
`lock` calls return an error that still contains the guard. `unwrap()` is the
usual response.) Everything in the previous two sections applies with one
difference: a conflict does not panic, it *waits*, because another thread may
release the lock. On the same thread, nobody will. The documentation of
`Mutex::lock` says: "The exact behavior on locking a mutex in the thread which
already holds the lock is left unspecified. However, this function will not
return on the second call (it might panic or deadlock, for example)."

So the examples below use `try_lock`, which returns
`Err(TryLockError::WouldBlock)` instead of waiting
(`code/examples/ch12_mutex_guards.rs`):

```rust
{{#include ../../code/examples/ch12_mutex_guards.rs:status}}
```

### Sharing a mutex between scoped threads

With `thread::scope` (Chapter 7), threads can borrow the mutex directly: the scope
guarantees that they finish before `lengths` is dropped, so no `Arc` is needed:

```rust
{{#include ../../code/examples/ch12_mutex_guards.rs:scoped}}
```

```text
{{#include ../../code/output/ch12_mutex_guards.txt:1:1}}
```

Each thread does its work (here, `word.len()`) *before* taking the lock, and the
guard is a temporary of the statement `lengths.lock().unwrap().push(...)`, so the
lock is held for the push and released at the `;`. Keeping the locked region
short is the main design rule for mutexes, and the temporary scope rules are how
you control it.

### Scoping a guard

When you need the lock for several statements, bind the guard and give it a
block:

```rust
{{#include ../../code/examples/ch12_mutex_guards.rs:block}}
```

```text
{{#include ../../code/output/ch12_mutex_guards.txt:2:3}}
```

`drop(jobs)` would release it just as well, earlier than the end of the block.
What does *not* work is `let _ = queue.lock().unwrap();` to "hold the lock for
the rest of the function": `_` binds nothing, so the guard is dropped at the end
of the statement (Chapter 5). The mistake is common enough that rustc has a
deny-by-default lint for it (`code/compile_fail/ch12_let_underscore_lock.rs`):

```rust
{{#include ../../code/compile_fail/ch12_let_underscore_lock.rs:underscore}}
```

```text
{{#include ../../code/compile_fail/ch12_let_underscore_lock.stderr}}
```

The first help is the fix you meant: bind the guard to a name that starts with an
underscore, such as `_guard`, which *is* a binding and keeps the guard until the
end of the scope.

### The `while let` trap with a mutex

The `while let` rule from the previous section turns into a deadlock with a
mutex:

```rust
{{#include ../../code/examples/ch12_mutex_guards.rs:while_let}}
```

```text
{{#include ../../code/output/ch12_mutex_guards.txt:4:6}}
```

The body runs with the queue locked. If it called `queue.lock()` to push a
follow-up job, the thread would wait for itself forever (or panic: the behavior is
unspecified). The fix is to take the job out in a statement of its own, so that the
guard dies at the `;` before the body runs:

```rust
{{#include ../../code/examples/ch12_mutex_guards.rs:loop_fix}}
```

```text
{{#include ../../code/output/ch12_mutex_guards.txt:7:8}}
```

The same applies to `match queue.lock().unwrap().pop() { .. }` in every edition,
and to `if let` in edition 2021.

> [!WARNING]
> A `RefCell` conflict panics, which you will notice in your tests. A `Mutex`
> conflict on the same thread may deadlock, and a lock held longer than needed
> only makes other threads wait, which you may not notice at all. Review every
> `match`, `while let` and extended `let` whose scrutinee or initializer calls
> `lock()`.

### Readers and writers: `RwLock`

`RwLock<T>` allows many readers or one writer, with `read()` and `write()`
returning `RwLockReadGuard<'_, T>` and `RwLockWriteGuard<'_, T>`. The rules for
the guards are the same (`code/examples/ch12_rwlock_static.rs`):

```rust
{{#include ../../code/examples/ch12_rwlock_static.rs:rwlock}}
```

```text
{{#include ../../code/output/ch12_rwlock_static.txt:1:2}}
```

Two read guards coexist, and a writer must wait until both are dropped. The write
happens through a temporary guard in a statement of its own, and the final read
guard is a temporary of the `println!` statement.

### Guards of a `static`

The lifetime in a guard's type is the lifetime of the borrow of the lock. When
the lock is a `static`, the borrow can be `'static`, and so can the guard:

```rust
{{#include ../../code/examples/ch12_rwlock_static.rs:static}}
```

```text
{{#include ../../code/output/ch12_rwlock_static.txt:3:3}}
```

`log_entries` returns a `MutexGuard<'static, Vec<String>>` with no lifetime
parameter in sight, because `LOG` lives for the whole program. Note what
`'static` does *not* mean here: the lock is held exactly as long as the guard
lives, which should still be short. `'static` describes the *mutex*, not the
critical section.

One tool you might look for is missing on stable Rust: there is no `MutexGuard::map`
to narrow a guard the way `Ref::map` does. In Rust 1.99, `MappedMutexGuard` and
the `map` functions of the lock guards are unstable (feature
`mapped_lock_guards`, tracking issue rust-lang/rust#117108). For a `Mutex`, use
the closure or the owned-copy design from the `RefCell` section, or return the
whole guard.

## `Cell`, `OnceCell` and invariance

### `Cell`: no references, no guards

`Cell<T>` takes the opposite approach to `RefCell`. It never lends a reference to
its contents through `&self`: you can copy the value out (`get`, for `T: Copy`),
put a value in (`set`), swap one in (`replace`) or take it out (`take`, for
`T: Default`). Since no reference into a `Cell` can exist while someone holds
only `&Cell<T>`, a `set` can never invalidate one, and nothing needs to be
counted. Here is a document that counts how often its summary is read
(`code/examples/ch12_cell_oncecell.rs`):

```rust
{{#include ../../code/examples/ch12_cell_oncecell.rs:document}}
```

`reads` is updated in `summary(&self)` with `get` and `set`: no guard, no
run-time check, no possible panic. The price is the restricted API: you cannot
call a method on the value *inside* the cell, only move whole values in and out.

### `Cell<&'a T>` and invariance

A `Cell` can hold a borrowed value, which makes its lifetime parameter part of the
picture. This cursor remembers the last token it saw, through `&self`:

```rust
{{#include ../../code/examples/ch12_cell_oncecell.rs:cursor}}
```

This works because every token comes from the same source text, `'src`. Chapter 4
showed why `Cell<T>` is *invariant* in `T`, and here is the consequence in a
setting of this chapter. The compiler rejects this program
(`code/compile_fail/ch12_cell_invariance.rs`):

```rust
{{#include ../../code/compile_fail/ch12_cell_invariance.rs:invariance}}
```

```text
{{#include ../../code/compile_fail/ch12_cell_invariance.stderr}}
```

`remember` asks for a slot and a token with the same `'a`. For a plain
`&&'static str`, the compiler could shrink the slot's type to `&&'a str`
(covariance), pick `'a` as short as `local`, and accept the call. For a
`Cell`, it may not: `slot` would then let `remember` store a short-lived `&str`
in a cell that the caller still believes to hold `&'static str`, and the caller
could read it after `local` is gone. Since the cell's type cannot change, `'a`
must be `'static`, and the label says so: "argument requires that `local` is
borrowed for `'static`". Invariance is how the type system protects every type
that can be written through a shared reference.

### `OnceCell`: plain references, no guard

`OnceCell<T>` is a cell that can be written once. Its key method has a signature
that may surprise you after the `RefCell` section:

```rust,ignore
impl<T> OnceCell<T> {
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T;
}
```

It returns a plain `&T` with the lifetime of `&self`: no guard. The `summary`
method of `Document` above uses it to compute the summary on first use and to
return a `&str` that borrows the document:

```rust
{{#include ../../code/examples/ch12_cell_oncecell.rs:use}}
```

```text
{{#include ../../code/output/ch12_cell_oncecell.txt:1:3}}
```

The summary is computed once, and two references to it are alive at the same
time as the `Cell` counter is updated. This is sound because of what `OnceCell`
does *not* offer: once the cell is full, nothing that takes `&self` changes the
contents. A second `get_or_init` returns the existing value. The methods that
can change or remove it (`take`, `get_mut`, `into_inner`) need `&mut self` or
ownership, and those conflict with every outstanding `&T` in the ordinary,
compile-time way. This program is rejected
(`code/compile_fail/ch12_oncecell_take.rs`):

```rust
{{#include ../../code/compile_fail/ch12_oncecell_take.rs:take}}
```

```text
{{#include ../../code/compile_fail/ch12_oncecell_take.stderr}}
```

So `OnceCell` needs only one bit of run-time state (empty or full). The aliasing
rule for the contents is left to the borrow checker, which can enforce it because
the dangerous operations require `&mut`. That is the design to aim for when you
build your own types: the more of the aliasing rule you express with `&` and
`&mut` in signatures, the less you have to check at run time.

## `thread_local!` and closure-scoped borrows

A `thread_local!` declares a value of which each thread has its own copy. The
declaration creates a `LocalKey<T>`, and the value is reached through methods of
the key (`code/examples/ch12_thread_local.rs`):

```rust
{{#include ../../code/examples/ch12_thread_local.rs:declare}}
```

```rust
{{#include ../../code/examples/ch12_thread_local.rs:access}}
```

```rust
{{#include ../../code/examples/ch12_thread_local.rs:threads}}
```

```text
{{#include ../../code/output/ch12_thread_local.txt}}
```

The worker thread sees only its own entry and its own `DEPTH`, which is still 0;
the main thread sees its two entries. `with_borrow` and `with_borrow_mut` (for
`LocalKey<RefCell<T>>`) and `get` and `set` (for `LocalKey<Cell<T>>`) are
shortcuts, stable since Rust 1.73.

### Why `with` takes a closure

Every access goes through a closure. The signature of `LocalKey::with`, verbatim
from the standard library, is:

```rust,ignore
pub fn with<F, R>(&'static self, f: F) -> R
where
    F: FnOnce(&T) -> R,
```

The key is `&'static`, but the value is not: each thread has its own, and it is
destroyed when that thread exits. A `&'static T` to it could outlive its thread. So `with` lends the reference to `f` instead. `FnOnce(&T) -> R` elides the
lifetime of `&T`, which makes the bound higher-ranked (Chapter 9):
`for<'a> FnOnce(&'a T) -> R`. `with` picks `'a` inside its body, and `R` is fixed
by the caller before that, so `R` cannot contain `'a`. Trying to return the
reference anyway is rejected (`code/compile_fail/ch12_thread_local_escape.rs`):

```rust
{{#include ../../code/compile_fail/ch12_thread_local_escape.rs:escape}}
```

```text
{{#include ../../code/compile_fail/ch12_thread_local_escape.stderr}}
```

This error has no code; it is the "lifetime may not live long enough" of closures.
The labels name two anonymous lifetimes: `'1` is the lifetime of the closure's
parameter `text` ("has type `&'1 String`"), and `'2` is the lifetime in the
closure's return type ("return type of closure is `&'2 str`"). Returning `text`'s
contents requires `'1` to outlive `'2`. But `'1` is chosen anew for each call,
inside `with`, and `'2` is part of the type `R`, chosen outside. Nothing can
make one outlive the other, which is the point: the borrow is confined to the
closure.

Even inside the closure, `with` makes no promise for the time while the thread is
being torn down: its documentation says that it "will `panic!()` if the key
currently has its destructor running, and it **may** panic if the destructor has
previously been run for this thread". The fallible variant `try_with` returns an
error instead.

### Lending instead of returning

`LocalKey::with` is one instance of a pattern you have now seen three times:
`History::with_last` in this chapter, `thread::scope` in Chapter 7, and
`LocalKey::with`. When the validity of a reference depends on something that must
happen *after* its last use (a guard released, a thread joined, a thread-local
torn down), the API cannot return the reference, because the caller could keep it
past that point. It lends the reference to a closure, and a higher-ranked bound
guarantees that the closure gives it back. The cost is a less direct calling
style; the gain is that misuse is a compile error instead of a panic, a deadlock
or a dangling pointer.

## Common confusions

**"`RefCell` turns off the borrow checker."** It moves one check to run time: the
conflict between guards of the same cell. Every guard is still an ordinary borrow
of the cell, and the cell is still subject to every compile-time rule
(`ch12_guard_outlives_cell.rs`).

**"A `Ref<'b, T>` can give me a `&'b T`."** It gives you a reference tied to the
guard *value*. That is what keeps the run-time count honest, and it is why
getters return `Ref<'_, T>`, take a closure, or return an owned copy.

**"`match cell.borrow().len()` releases the guard, since `len()` returned a
number."** The guard is a temporary of the scrutinee and lives until the end of
the `match`, whatever the type of the scrutinee's value. The same holds for
`while let`; `if let` releases it before `else` in edition 2024.

**"`let _ = mutex.lock()` holds the lock."** It releases it immediately, and rustc
rejects it for that reason. `let _guard = mutex.lock().unwrap();` holds it.

**"A `MutexGuard<'static, T>` holds the lock forever."** It holds the lock until
it is dropped. The `'static` is the lifetime of the borrow of the mutex, possible
because the mutex is a `static`.

**"`Cell` is a cheaper `RefCell`."** It is a different contract: no references
to the contents at all. When you need to call a method on the contents in place,
you need `RefCell` (or a lock); when you only move values in and out, `Cell` is
simpler and cannot panic.

## Summary

- Interior mutability allows mutation through `&T`. It keeps the
  readers-or-writer rule, enforced at run time (`RefCell`, `Mutex`, `RwLock`) or by
  an API that cannot violate it (`Cell`, `OnceCell`). It does not remove lifetimes.
- `RefCell::borrow(&self) -> Ref<'_, T>`: the guard is a compile-time borrow of the
  cell and a run-time borrow of its contents. Conflicts between guards panic
  ("RefCell already borrowed", "RefCell already mutably borrowed"); conflicts with
  the cell itself are compile errors.
- References obtained through a guard live no longer than the guard. A getter
  cannot return `&T` from a `RefCell` (E0515); return a guard narrowed with
  `Ref::map` or `Ref::filter_map`, lend the data to a closure, or return an owned
  copy.
- A guard that is not bound to a variable is a temporary: it lives to the end of
  a `match`, of each `while let` iteration, or of the block when extended; in
  edition 2024 an `if let` releases it before `else`, and a tail expression before
  the block's locals.
- `MutexGuard` and the `RwLock` guards follow the same rules; a conflict waits
  instead of panicking, and on the same thread never returns. Keep critical
  sections short with statement-level temporaries, blocks or `drop`; never
  `let _ =` a guard. Guards of a `static` lock are `'static`.
- `Cell` never lends references, so it needs no guard; `Cell<T>` is invariant in
  `T`. `OnceCell::get_or_init` returns `&T` tied to `&self` because the contents
  never change through `&self` once set.
- `LocalKey::with` lends `&T` to a closure with a higher-ranked bound, so the
  reference cannot escape the call. Lending instead of returning is the general
  pattern when a reference must not outlive a cleanup step.

## Exercises

1. Add to `History` (from `ch12_ref_map.rs`) a method
   `fn longest(&self) -> Option<Ref<'_, str>>` that returns the longest entry.
   Then write a caller that holds the result in a variable while calling
   `history.entries.borrow_mut()`, and observe the panic. Change the caller, not
   the method, so that it works.
   *Hint:* `Ref::filter_map` with `iter().max_by_key(..)`; the caller needs a
   shorter scope for the guard.

2. A work queue is a `RefCell<Vec<String>>`. Write a loop that pops jobs and, for
   every job that ends with `!`, pushes a follow-up job without the `!`. Write it
   first with `while let Some(job) = queue.borrow_mut().pop()` and run it; then fix
   it.
   *Hint:* the guard of a `while let` scrutinee lives for the whole body; pop in a
   `let` statement inside a `loop`.

3. Rewrite `History` with a `Mutex<Vec<String>>` so that it can be shared by
   threads in a `thread::scope`. Which of `entries`, `last`, `with_last` and
   `last_owned` can you keep with the same shape, and what must change?
   *Hint:* there is no stable `MutexGuard::map`; a method can still return the
   whole `MutexGuard<'_, Vec<String>>`.

4. Declare a thread-local `RefCell<Vec<String>>` of configuration lines and write
   `fn with_config<R>(f: impl FnOnce(&[String]) -> R) -> R`. Then try to write
   `fn config() -> &'static [String]` on top of it, read the error, and explain in
   one sentence which lifetime is chosen by whom.
   *Hint:* compare with `ch12_thread_local_escape.rs`; the closure's parameter
   lifetime is chosen inside `with`.

5. Give `Document` a field `words: OnceCell<Vec<String>>` filled on first use, and
   a method `fn words(&self) -> &[String]`. Explain why this needs no guard, then
   explain why `fn reset(&self)` that empties the cell cannot be written, while
   `fn reset(&mut self)` can (with `OnceCell` alone).
   *Hint:* look at which `OnceCell` methods take `&self` and which take
   `&mut self`.
