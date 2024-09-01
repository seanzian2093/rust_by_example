#![allow(dead_code)]

// A struct
struct Container(i32, i32);

// Define a trait that uses two generic types:`A` and `B`
    // - `A` and `B` are defined in the trait via the `type` keyword.
    // - `type` in this context is different from `type` when used for aliases
trait Contains {
    // - define generic types here which methods will be able to utilize.
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are
        // - If the `input` type is `Container(i32, i32)`, the `output` types are determined as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// Withusing associated types
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
// Without using associated types, as in `problem.rs`
    // - `Contains` would be defined as `Contains2` 
trait Contains2<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}
    // - `difference` would be defined as `difference2`
fn difference2<A, B, C>(container: &C) -> i32 where
    C: Contains2<A, B> { 
        container.last() - container.first()
    }



pub fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}