// Multiple bounds for a single type can be applied with a +. Like normal, different types are separated with ,.
#![allow(dead_code)]
use std::fmt::{Debug, Display};

// Define a function `compare_prints` that takes a generic type `T` which must implement trait `Display` and `Debug`
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

// Define a function `compare_types` that takes two generic types `T` and `U`, which
// -`T` must implement trait `Debug`
// -`U` must implement trait `Debug`
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

pub fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    // vec implements both `Display` and `Debug`
    compare_prints(&string);
    // array does not implement `Display`
    // compare_prints(&array);
    // TODO ^ Try uncommenting this.

    // both array and vec implement `Debug`
    compare_types(&array, &vec);
}
