// if omit the `as ` part, it is quivalent to `use crate::modules::visibility::main as main`
use crate::modules::visibility::main as other_main;
pub fn main() {
    println!("\n Calling other_main()");
    other_main();

    println!("\nEntering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::modules::struct_visibility::main as other_main;

        // `use` bindings have a local scope. In this case, the shadowing of `function()` is only in this block.
        other_main();

        println!("Leaving block");
    }
}
