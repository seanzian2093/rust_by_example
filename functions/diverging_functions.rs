// Diverging functions never return. They are marked using !, which is an empty type.
// - e.g. foo never return
#[allow(dead_code)]
fn foo() -> ! {
    panic!("This call never returns.");
}
// - empty type cannot be instantiated, because the set of all possible values this type can have is empty.
// - as opposed to all the other types
// - it is different from the () type, which has exactly one possible value, i.e., `()`
// - e.g. `some_fn` returns `()`
fn some_fn() {
    ()
}
pub fn main() {
    let _a: () = some_fn();
    // since `some_fn` returns, so code after it will be executed
    println!("This function returns and you can see this line.");
    // `panic!` never returns
    // - need to figure out how to fix compiler error
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    //  The main advantage of this type is that it can be cast to any other one and
    // - therefore used at places where an exact type is required,
    // - e.g. in match branches
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // - the return type of this match expression must be u32
            // - because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // - the "i" variable is of type u32, which is perfectly fine.
                true => i,
                // - on the other hand, the "continue" expression does not return u32, but it is still fine
                // - because it never returns and
                // - therefore does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
