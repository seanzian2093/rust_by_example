pub mod dry;
pub mod dsl;
pub mod syntax;
pub mod variadic_interfaces;
// WHy macros are useful
// - Don't repeat yourself. There are many cases where you may need similar functionality in multiple places but with different types. Often, writing a macro is a useful way to avoid repeating code. (More on this later)
// - Domain-specific languages. Macros allow you to define special syntax for a specific purpose. (More on this later)
// - Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is println! which could take any number of arguments, depending on the format string. (More on this later)

// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

pub fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!()
}
