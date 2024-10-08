pub mod boxing_errors;
pub mod iterate_over_results;
pub mod own_error_type;
pub mod pull_result_from_option;
pub mod use_question_mark;
pub mod wrapping_errors;
// The previous examples have always been very convenient
// - Results interact with other Results and
// - Options interact with other Options.
// But in reality
// - an Option needs to interact with a Result or
// - a Result<T, Error1> needs to interact with a Result<T, Error2>
// - in those cases, we want to manage our different error types in a way that makes them composable and easy to interact with.
// - e.g., two instances of unwrap generate different error types. Vec::first returns an Option, while parse::<i32> returns a Result<i32, ParseIntError>:
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

pub fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}
