#![allow(dead_code)]
use std::iter;
use std::vec::IntoIter;
    
// impl Trait can be used in two locations:
    // - as an argument type
    // - as a return type
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}
// parse_csv_document2 is generic, allowing it to take any type which implements BufRead, such as BufReader<File> or [u8]
    // - but it's not important what type R is, and R is only used to declare the type of src,
    // - so the function can also be written as
    // - we cannot provide generic type, <T> to it since none in its definition
fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

// If your function returns a type that implements MyTrait, you can write its return type as -> impl MyTrait.
    // - this can help simplify your type signatures quite a lot!

// This function combines two `Vec<i32>` and returns an iterator over it.
    // - Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
    // - Look how much simpler it is!
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Some Rust types can't be written out. For example, every closure has its own unnamed concrete type
    // - before impl Trait syntax, you had to allocate on the heap in order to return a closure
    // - but now you can do it all statically
    // - e.g.,  returns a function that adds `y` to its input

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

// We can also use impl Trait to return an iterator that uses map or filter closures
    // - this makes using map and filter easier. Because closure types don't have names, you can't write out an explicit return type 
    // - if your function returns iterators with closures. But with impl Trait you can do this easily

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}


pub fn main() {

    let _res = parse_csv_document::<std::io::Empty>(std::io::empty());
    // we cannot provide generic argument because it does not take any
    // parse_csv_document2::<std::io::Empty>(std::io::empty());
    // - instead we can do 
    let _res = parse_csv_document2(std::io::empty());

    // As return type
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    // return a closure
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
    
    // return iterators
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);

}