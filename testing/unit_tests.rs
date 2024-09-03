// Most unit tests 
    // - go into a tests mod with the #[cfg(test)] attribute
    // - test functions are marked with the #[test] attribute.  
// Tests fail when something in the test function panics. There are some helper macros:
    // - assert!(expression) - panics if expression evaluates to false.
    // - assert_eq!(left, right) - testing left and right expressions for equality 
    // - assert_ne!(left, right) - testing left and right expressions for inequality
// in Rust 2018, your unit tests can return Result<()>, which lets you use ? in them
    // - this can make them much more concise.

// To check functions that should panic under certain circumstances, use attribute #[should_panic]
    // - this attribute accepts optional parameter expected = with the text of the panic message
    // - if your function can panic in multiple ways, it helps make sure your test is testing the correct panic.

// Tests can be marked with the #[ignore] attribute to exclude some tests
    // - Or to run them with command cargo test -- --ignored
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn main() {
    println!("Run \'cargo test\' instead for all tests");
    println!("Run \'cargo test testing\' instead for tests in a module, e.g. testing module");
    println!("Run \'cargo test test_any_panic\' instead for a specific test, e.g. test_any_panic");
    println!("Run \'cargo test -- --ignored\' instead for all ignored tests");
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    // a test that returns
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        // use `?` operator
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    // test panic
    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    // Ignored test
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}