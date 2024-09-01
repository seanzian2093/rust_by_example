pub mod map_for_result;
pub mod aliases_for_result;
pub mod early_return;
pub mod question_mark_operator;

use std::num::ParseIntError;
// Result is a richer version of the Option type that describes possible error instead of possible absence.
    // - i.e., Result<T, E> could have one of two outcomes:
        // - Ok(T): An element T was found
        // - Err(E): An error was found with element E

// Like Option, Result has many methods associated with it
    // - e.g., unwrap() either yields the element T or panics.
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}


// main() can return a Result
pub fn main() -> Result<(), ParseIntError> {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // below let statement will cause program to panic
    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}