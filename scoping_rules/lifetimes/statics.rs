// Rust has a few reserved lifetime names. One of those is 'static. You might encounter it in two situations:
    // - a reference with 'static lifetime:
        // - e.g., `let s: &'static str = "hello world";`
    // - 'static as part of a trait bound:
        // - e.g., `fn generic<T>(x: T) where T: 'static {}`

// As a reference lifetime 'static indicates that the pointee of the reference lives for the remaining lifetime of the running program.
    // - It can still be coerced to a shorter lifetime.

// - There are two common ways to make a variable with 'static lifetime, and
    // - both are stored in the read-only memory of the binary:
    // - Make a constant with the static declaration.
    // - Make a string literal which has type: &'static str.

// Since 'static references only need to be valid for the remainder of a program's life, they can be created while the program is executed
    // - e.g., uses Box::leak to dynamically create 'static references
    // - in that case it definitely doesn't live for the entire duration, but only for the leaking point onward

// When used in trait bound, 'static means the type does not contain any non-static references.
    // - Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.
    // - this means that any owned data always passes a 'static lifetime bound, but
    // - a reference to that owned data generally does not
#![allow(dead_code)]

extern crate rand;
use rand::Fill;
use std::fmt::Debug;


fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    // create a static from here
    Box::leak(boxed)
}

// A function that takes a 'static argument
fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

    // - returns a reference to `NUM` where its `'static` lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference can no longer be used
            // - but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second);

    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // Error  &i only has the lifetime defined by the scope of main(), so it's not 'static:
    // print_it(&i);

}