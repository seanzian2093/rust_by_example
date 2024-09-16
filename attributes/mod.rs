#![allow(unused_variables)]
pub mod cfg;
// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
// conditional compilation of code
// set crate name, version and type (binary or library)
// disable lints (warnings)
// enable compiler features (macros, glob imports, etc.)
// link to a foreign library
// mark functions as unit tests
// mark functions that will be part of a benchmark
// attribute like macros

// Attributes look like #[outer_attribute] or #![inner_attribute], with the difference between them being where they apply.
// - `#[outer_attribute]` applies to the item immediately following it.
// - Some examples of items are: a function, a module declaration, a constant, a structure, an enum.
// - here is an example where attribute #[derive(Debug)] applies to the struct Rectangle:
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}
// - `#![inner_attribute]` applies to the enclosing item (typically a module or a crate).
// - placed at top of source file
// - in other words, this attribute is interpreted as applying to the entire scope in which it's placed.
// - here is an example where `#![allow(unused_variables)]` applies to the whole crate (if placed in main.rs):

// The compiler provides a dead_code lint that will warn about unused functions. An attribute can be used to disable the lint.
// - including unused fields of struct
// - `#[allow(dead_code)]`
pub fn main() {
    let x = 3; // This would normally warn about an unused variable.
}
