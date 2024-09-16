// The most basic way of handling mixed error types is to just embed them in each other
use std::num::ParseIntError;

// - e.g. return type is a Result embedded in an Option
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

//  - Or we want to stop processing on errors (like with ?) but keep going when the Option is None
// - A couple of combinators come in handy to swap the Result and Option.
// - `map_or` for `Option` does follow
// - if None, returning the provided default value, i.e. 1st argument otherwise
// - if Some(), applying the provided function/closure, i.e., wnd argument to contained value in Some
fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

pub fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    // Error 1: the input vector is empty
    println!("The first doubled is {:?}", double_first(empty));
    // Error 2: the element doesn't parse to a number
    println!("The first doubled is {:?}", double_first(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("\nThe first doubled is {:?}", double_first2(numbers));
    println!("The first doubled is {:?}", double_first2(empty));
    println!("The first doubled is {:?}", double_first2(strings));
}
