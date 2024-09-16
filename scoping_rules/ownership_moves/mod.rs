pub mod mutability;
pub mod partial_moves;
// Variables are in charge of freeing their own resources, resources can only have one owner
// - this prevents resources from being freed more than once, an undefined behavior
// - not all variables own resources (e.g. references).
// - The ownership of the resources is transferred. In Rust-speak, this is known as a move.
// - when doing assignments (let x = y) or
// - passing function arguments by value (foo(x))

// - After moving resources, the previous owner can no longer be used
// - this avoids creating dangling pointers, using which is an undefined behavior

// This function takes ownership of its argument, a `Box<i32>` type, i.e., heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is out of scope so is destroyed and the memory freed
}

pub fn main() {
    // integers are stored on stack
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    // - when `Copy` is implemented, e.g., for `u32`
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // - Both are now pointers to the same heap allocated data
    // - but `b` now owns it.

    // - Error! `a` can no longer access the data, because it no longer owns the heap memory
    //println!("a contains: {}", a);
    // - TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    // -
    destroy_box(b);

    // Since the heap memory has been freed at this point
    // - this action would result in dereferencing freed memory, but it's forbidden by the compiler
    // - Error! Same reason as the previous Error
    // println!("b contains: {}", b);
    // - TODO ^ Try uncommenting this line
}
