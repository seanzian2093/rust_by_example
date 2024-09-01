// `panic` is the simplest error handling mechanism
    // -  it prints an error message
    // -  starts unwinding the stack
    // - and usually exits the program
    // - e.g., Here, we explicitly call panic on our error condition:
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

pub fn main() {
    drink("water");
    // The first call to drink works. The second panics and thus the third is never called.
    drink("lemonade");
    drink("still water");
}