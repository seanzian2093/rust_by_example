pub mod mutability;
pub mod aliasing;
pub mod ref_pattern;
//  When we wish to access data without taking ownership over it, we can pass objects by reference (&T).
    // - instead of passing objects by value (T), 
    // - the compiler statically guarantees (via its borrow checker) that references always point to valid objects. 
    // - i.e., while references to an object exist, the object cannot be destroyed.

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32, i.e., takes a ref to i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

pub fn main() {
    // Create a boxed i32 in the heap, and a i32 on the stack
        // - numbers can have arbitrary underscores added for readability
        // - 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
        // - so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box_32` and be destroyed
    eat_box_i32(boxed_i32);
}