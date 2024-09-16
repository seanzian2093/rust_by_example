// An Iter::map operation might fail we need a strategy to handle this

pub fn main() {
    let strings = vec!["tofu", "93", "18"];
    // this `map` will fail because "tofu" cannot be parsed into `i32`
    // - both error and Oks are collected in the vector, numbers
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // `filter_map` calls a function and filters out the results that are None
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        // `.ok` converts its caller Result<T,E> to Option<T>
        // - discard error if any, i.e. None if error
        // `.filter_map`
        // - takes a closure/function,
        // - returns an iterator that yields `value` for which the provided clousre returns `Some(value)`
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("\nResults: {:?}", numbers);

    // fail entire operation with `collect` by annotating number as `Result<Vec<_>, _>`
    // - `Result` implements `FromIterator` so
    // - a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>)
    // - i.e., Once an Result::Err is found, the iteration will terminate.
    // - This same technique can be used with Option
    // - numbers is either an error or a vector
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("\nResults: {:?}", numbers);

    // Collect all valid values and failures with partition()
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        // `partition` takes a predicate that returns a bool
        .partition(Result::is_ok);
    println!("\nNumbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
    // pull values out of Ok or Error
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("\nNumbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
