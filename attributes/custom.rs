// Some conditionals like target_os are implicitly provided by rustc, but 
// custom conditionals must be passed to `rustc`` using the `--cfg` flag.

// try 
    // - `rustc --cfg some_condition custom.rs && ./custom`
        // - we will see `condition met!`
    // - `rustc custom.rs && ./custom`
        // - we will see compilor error because `conditional_function` is not compiled due to `some_condition` not met
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}