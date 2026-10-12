Part II turns to the lifetimes that appear in the signatures of real APIs, and
to the many places where the compiler writes a lifetime for you. The anonymous
lifetime `'_` makes hidden borrows visible; `'static` turns out to mean two
different things, one of which is far more common than the other; trait objects
and `impl Trait` carry lifetime bounds and captures that are easy to miss and
changed in edition 2024; and higher-ranked trait bounds, written `for<'a>`,
describe callbacks and traits that must work for every lifetime, including
lifetimes that exist only inside the function that receives them.
