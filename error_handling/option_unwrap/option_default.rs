// There is more than one way to unpack an Option and fall back on a default if it is None
    // - to choose the one that meets our needs, we need to consider the following:
        // - do we need eager or lazy evaluation?
        // - do we need to keep the original empty value intact, or modify it in place?

// `or()` is
    // - caller is an Option
    // - takes an Option
    // - return caller if caller contains value
        // - otherwise return the argument option

    // - chainable and
    // - eagerly evaluates its argument 
        // -  therefore the variable passed to or is moved
    // - e.g.

// `or_else()` is chainable, evaluates lazily, keeps empty value intact
    // - caller is an Option
    // - takes a closur/function as parameter, which returns an Option
    // - return caller if caller contains value
        //-  otherwise call the closure/function and return the resultant option


// `get_or_insert()` evaluates eagerly, modifies empty value in place
    // - caller is an Option
    // - takes a Generic type as parameter
    // - if caller is None, insert/move the argument to the caller and
        // - return a mutable ref to the contained value
#![allow(dead_code)]
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }


pub fn main() {
    // or
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // first_available_fruit is Some(Orange)

    // `or` moves its argument.
        // - in the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
        // - but the variable named `apple` has been moved regardless, and cannot be used anymore.
    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error

    // or_else
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // Providing kiwi as fallback
    // first_available_fruit is Some(Kiwi)

    // get_or_insert
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let orange = Fruit::Orange;

    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // first_available_fruit is: Apple
    // my_fruit is: Some(Apple)
    // - apple is moved
    // println!("Variable named `apple` is moved: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error

    let second_available_fruit = my_fruit.get_or_insert(orange);
    println!("second_available_fruit is: {:?}", second_available_fruit);
    // orange is moved
    // println!("Variable named `orange` is moved: {:?}", orange);
}