// extern crate rary; // May be required for Rust 2015 edition or earlier

// Run below command in terminal
    // - Where `library.rlib`` is the path to the compiled library, assumed that it's in the same directory here:
    // - `rustc executable.rs --extern rary=library.rlib && ./executable` we will see
    // called rary's `public_function()`
    // called rary's `indirect_access()`, that
    // > called rary's `private_function()`


fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}