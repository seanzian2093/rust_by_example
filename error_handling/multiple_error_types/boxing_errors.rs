// A way to write simple code while preserving the original errors is to Box them
// - the drawback is that the underlying error type is only known at runtime and not statically determined.
// - the standard library helps in boxing our errors by
// - having `Box` implement conversion from
// - any type that implements the `Error` trait into
// - the trait object `Box<Error>`
// -  via `From`

use std::error;
use std::fmt;

// Change the alias to use `Box<dyn error::Error>`.
// - so in this scope, all `Result<T>` is `Result<T, Box<dyn error::Error>>`;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
// Implement `Error` for `EmptyVec`
impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // - `ok_or_else(err)` changes the caller Option<T> to Result<T,E> where
        // - Some(v) to Ok(v) and None to Err(err()) where err() return an error type
        // - since `ok_or_else` requires an Error type, `.into()` converts its caller to the Error type
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
