// Rust chooses how to capture variables on the fly mostly without type annotation 
    // - When taking a closure as an input parameter, the closure's complete type must be annotated 
    // - using one of a few traits, and they're determined by what the closure does with captured value. 
    // - In order of decreasing restriction, they are:
        // - Fn: the closure uses the captured value by reference (&T)
        // - FnMut: the closure uses the captured value by mutable reference (&mut T)
        // - FnOnce: the closure uses the captured value by value (T)
        // - On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

// For instance, consider a parameter annotated as FnOnce. 
    // - This specifies that the closure may capture by T, or more restrictive ones, &mut T, or &T, 
        // - this is because if a move is possible, then any type of borrow should also be possible. 
        // - the reverse is not true. If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed. However, &T is allowed.
    // - but the compiler will ultimately choose based on how the captured variables are used in the closure.

// When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, 
    // - meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type. 
    // - This type is assigned to the variable which is stored until calling.

// If a function takes a closure as parameter, then any function that satisfies the trait bound of that closure can be passed as a parameter.
    // - `call_me` is a function which takes a generic `F` argument bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}
    // - a wrapper function satisfying the `Fn` bound so `call_me` can take `function` as its argument
        // - in the `main`` we will define a closure satisfying `Fn` bound and pass it to `call_me``
fn function() {
    println!("I'm a function!");
}

// Returning closures as output parameters is possible. 
    // - closures are anonymous types, by definition, unknown, so we have to use `impl Trait` to return them.
    // - The valid traits for returning a closure are:
        // - `Fn`, `FnMut`, `FnOnce`
    // - the `move` keyword must be used, which signals that all captures occur by value. 
        // - because any captures by reference would be dropped as soon as the function exited, leaving invalid references in the closure.
        // - e.g. `create_fn` is a function return a closure with `Fn` trait
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

        // - `move` must be used
    move || println!("This is a: {}", text)
}
        // - e.g. `create_fnmut` is a function return a closure with `FnMut` trait
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}
        // - e.g. `create_fnonce` is a function return a closure with `FnOnce` trait
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}
        

// A function which takes a closure as an argument and calls it.
    // - <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // - the closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // - the closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

pub fn main() {
    use std::mem;

    // A non-copy type.
    let greeting = "hello";
        // - `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // `diary` capture 2 variables based on how they are used in closure body 
        // - `greeting` by reference and 
        // - `farewell` by value.
    let diary = || {
        // - `println!` uses `greeting` by reference, i.e., requires `Fn`.
        println!("I said {}.", greeting);

        // - `push_str` is a mutation that forces `farewell` to be captured by mutable reference, i.e., requires `FnMut`.
            // - for now
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // - manually calling drop forces `farewell` to be captured by value, i.e., requires `FnOnce`.
            // - for now, `FnOnce` supersedes `FnMut`
        mem::drop(farewell);
    };

        // - Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound so below definition does following
        // - apture `x` into an anonymous type 
        // - implement `Fn` for this type
        // - store this type to in `double`
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));


    // Function as parameter
        // - define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

        // - `call_me` accepts `closure` and `function`
    call_me(closure);
    call_me(function);

    // Clousre as output parameter, i.e., return type
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}