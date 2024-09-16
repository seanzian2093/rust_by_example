// A function `age` which returns a `u32`.
fn age(arg: u32) -> u32 {
    arg + 1
}

fn some_number(arg: u32) -> Option<u32> {
    Some(arg + 1)
}

pub fn main() {
    println!("\nTell me what type of person you are");
    match_age(0);

    println!("\nTell me what type of person you are");
    match_age(11);

    println!("\nTell me what type of person you are");
    match_age(12);

    println!("\nTell me what type of person you are");
    match_age(18);

    println!("\nTell me what type of person you are");
    match_age(19);

    println!("\nGive me a number to match");
    match_some_number(41);
    match_some_number(42);
}

fn match_age(arg: u32) {
    match age(arg) {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age would the child be?
        // Instead, bind to `n` for the sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn match_some_number(arg: u32) {
    match some_number(arg) {
        // Got `Some` variant, match if its value, bound to `n`, is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}
