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

pub fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}