#![allow(dead_code)]
// `Debug` is a trait which implements the print marker: `{:?}`.
// `Display` is a trait that if a type implements it, that type can be used in `println!`
use std::fmt::{Display, Debug};

// Define a function `printer` that takes a generic type `T` which must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// Define a struct whose only field must be a type that implements `Display`
struct S<T: Display>(T);

// Define a trait that will be implemented on various types
trait HasArea {
    fn area(&self) -> f64;
}
    // - a function for trait `HasArea` that implemented on `Rectangle`
impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

// Implement/derive `Debug` trait for `Rectangle`
#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
// Does not implement/derive `Debug` trait for `Triangle`
struct Triangle  { length: f64, height: f64 }

// Define a function that takes the generic `T` that must implement `Debug`. Regardless of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// Define a function that takes the generic `T` that must implement `HasArea`. 
    // - Any type which meets the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

// A consequence of how bounds work is that even if a trait doesn't include any functionality, you can still use it as a bound
struct Cardinal;
struct BlueJay;
struct Turkey;

    // - an empty trait
trait Red {}
    // - another empty trait
trait Blue {}
    // - implement the empty traits
impl Red for Cardinal {}
impl Blue for BlueJay {}

    // - These functions are only valid for types which implement these traits.
        // - The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }


pub fn main() {
    // Error! `Vec<T>` does not implement `Display`. This specialization will fail.
    // let s = S(vec![1]);

    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    // `Rectangle` implements `Debug` so `print_debug` works on instance of it
    print_debug(&rectangle);
    // `Rectangle` implements `HasArea` so `area` works on instance of it
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`. 


    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    // - below do not work because of bounds
    // println!("A turkey is {}", red(&_turkey));
    // println!("A turkey is {}", blue(&_turkey));

    // println!("A bluejay is {}", red(&blue_jay));
    // println!("A cardinal is {}", blue(&cardinal));
    // ^ TODO: Try uncommenting this line.
}