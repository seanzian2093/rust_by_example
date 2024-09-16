// Generally, we want to return the error to the caller so it can decide what is the right way to respond to error
use std::num::ParseIntError;

// As with `Option`, we can use combinators such as `map()`.
// - this function is otherwise identical to the one above and reads:
// - Multiply if both values can be parsed from str, otherwise pass on the error to caller
// - we must know the returned error type beforehand and specify it in return type of calling function
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);
}
