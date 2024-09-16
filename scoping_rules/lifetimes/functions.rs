// Some lifetime patterns are overwhelmingly common and so the borrow checker will allow you to omit them to save typing and to improve readability
// - this is known as elision. Elision exists in Rust solely because these patterns are common.
// Ignoring elision, function signatures with lifetimes have a few constraints:
// - any reference must have an annotated lifetime.
// - any reference being returned must have the same lifetime as an input or be static.
// - returning references without input is banned if it would result in returning references to invalid data

// One input reference with lifetime `'a` which must live at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes.
// - in this case, it would be fine for both to have the same lifetime `'a`, but
// - in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// - however, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// - here, `&String::from("foo")` would create a `String`, followed by a reference.
// - then the data is dropped upon exiting the scope, leaving a reference to invalid data to be returned.

// Methods are annotated similarly to functions
struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    // methods
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
