Part III applies the model to the parts of Rust where lifetimes are hardest to
see: generic associated types and lending iterators, async code, interior
mutability and its guards, and `unsafe` code, where you take over the job of
the borrow checker. It closes with the question every library author eventually
faces: when should a type borrow, when should it own, and how many lifetime
parameters does it need?
