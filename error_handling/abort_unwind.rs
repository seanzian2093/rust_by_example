// The previous section illustrates the error handling mechanism panic
// - Different code paths can be conditionally compiled based on the panic setting
// - The current values available are `unwind` and `abort`.
// - use `rustc  lemonade.rs -C panic=abort` to specify `panic` argument
// - use `rustc  lemonade.rs -C panic=unwind` to specify `panic` argument

// Building on the prior lemonade example, we explicitly use the panic strategy to exercise different lines of code.

// - use `cfg!(panic=)` macro to run code at run-time, conditionally
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}
// use `#cfg[panic=]` attribute to compile code conditionally
// - Depending on `panic` argument, `ah()`s will be compiled conditionally
#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!");
}

fn drink2(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

pub fn main() {
    drink("water");
    drink("lemonade");

    drink2("water");
    drink2("lemonade");
}
