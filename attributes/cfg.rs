// Configuration conditional checks are possible through two different operators:
// - the cfg attribute: `#[cfg(...)]` in attribute position
// - enables conditional compilation, i.e., remove code from compilation
// - the cfg! macro: `cfg!(...)` in boolean expressions
// - conditionally evaluates to true or false literals allowing for checks at run-time.
// - does not remove code
// - both utilize identical argument syntax.

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

pub fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
