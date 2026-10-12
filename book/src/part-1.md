Part I builds the model that every later chapter relies on. A lifetime is a
region of a function body, not a span of time; a signature is a contract that
the body and every caller are checked against; and the compiler fills in the
lifetimes you leave out according to a small set of fixed rules. Chapter 1
introduces regions and non-lexical lifetimes, Chapter 2 asks whose borrow a
method returns, Chapter 3 takes the elision rules apart, Chapter 4 explains how
lifetimes of different lengths fit together through subtyping and variance, and
Chapter 5 looks at the everyday mechanics that the model has to account for:
reborrows, temporaries, drops, and the few places where today's borrow checker
is stricter than it needs to be.
