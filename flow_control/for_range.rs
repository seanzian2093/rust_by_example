// The for in construct can be used to iterate through an Iterator.

// - One of the easiest ways to create an iterator is to use the range notation `a..b`.
// - `a..b` yields values from a (inclusive) to b (exclusive) in steps of one
// - `a..=b` yields values from a (inclusive) to b (inclusive) in steps of one
// - by default the for loop will apply the `into_iter`` function to the collection but there are more
// - `iter` borrows immutable
// - `into_iter` consume collection and take ownership
// - `iter_mut` borrows mutably

pub fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        // or
        // for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    // iter
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // - names is unusable after into_iter
    // println!("names: {:?}", names);

    // iter_mut
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
