// Sometimes we just want the simplicity of unwrap without the possibility of a panic
    // - `unwrap()` panicks at runtime if when caller Result contains Err
    // - as illustarted in `mod.rs::main`
// ? is almost1 exactly equivalent to an unwrap but
    // - returns the Err instead of panicking on it 

// Before there was ?, the same functionality was achieved with the try! macro
    // - the ? operator is now recommended, but you may still find try! when looking at older code
    // - e.g., the same multiply function from the previous example would look like `multiply2` using try!:
        // - since 2018 edition of Rust, `try!` is depreciated and `try` is reserved as keyword

// fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     let first_number = try!(first_number_str.parse::<i32>());
//     let second_number = try!(second_number_str.parse::<i32>());

//     Ok(first_number * second_number)
// }

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}


fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}