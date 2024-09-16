// In Rust, many of the operators can be overloaded via traits
// - that is, some operators can be used to accomplish different tasks based on their input arguments
// - this is possible because operators are syntactic sugar for method calls
// - e.g., the `+` operator in a + b calls the `add` method (as in `a.add(b)`)
// - this add method is part of the `Add` trait. Hence, the + operator can be used by any implementor of the Add trait.
// - a list of the traits, such as Add, that overload operators can be found in `core::ops`

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// - we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// - following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// - here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// - this block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

pub fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
