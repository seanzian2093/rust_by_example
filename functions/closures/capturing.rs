pub fn main() {
    use std::mem;

    let color = String::from("green");

    // when using closures directly, Rust decides how to capture values from environment based how they are used
    // - When taking a closure as an input parameter, the closure's complete type must be annotated
    // - discussed in `parameters` module

    // A closure to print `color` which
    // - immediately borrows (`&`) `color` and
    // - `println!` only requires arguments by immutable reference so it doesn't impose anything more restrictive.
    // - stores the borrow and closure in the `print` variable.
    // - It will remain borrowed until `print` is used the last time.
    let print = || println!("`color`: {}", color);

    // - Call the closure using the borrow.
    print();

    // - `color` can be borrowed immutably again, because the closure only holds an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // - A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either
    // - mutable ref, i.e., `&mut count` or
    // - ownership, i.e., `count`
    // - but `&mut count` is less restrictive so it takes that, i.e., immediately borrows `count`.
    // - a `mut` is required on `inc` because a `&mut` is stored inside.
    // - calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // - Call the closure using a mutable borrow.
    inc();

    // - The closure still mutably borrows `count` because it is called later, i.e., `inc()` in line 43
    // - so an attempt to reborrow will lead to an error.
    // - TODO: try uncommenting this line.
    // let _reborrow = &count;
    inc();

    // - The closure no longer needs to borrow `&mut count`. Therefore, it is possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value.
    // - a copy type, i.e., i32, would copy into the closure leaving the original untouched.
    // - non-copy must move and so `movable` immediately moves into the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // - `consume` consumes the variable so this can only be called once.
    consume();
    // - TODO: Try uncommenting this line.
    // consume();

    // Using keyword `move`` before vertical pipes `||` forces closure to take ownership of captured variables
    // - `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];
    // - `.contains` takes a ref to `haystack` but the keyword `move` forces the closure to take ownership of `haystack`
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // - Uncommenting above line will result in compile-time error
    // - because borrow checker doesn't allow re-using variable after it has been moved.
    // println!("There're {} elements in vec", haystack.len());

    // Removing `move` from closure's signature will cause closure to borrow _haystack_ variable immutably,
    // - hence _haystack_ is still available and uncommenting above line will not cause an error.
}
