#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

pub fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`
    match_temperature(temperature);

    let temperature = Temperature::Celsius(29);
    match_temperature(temperature);

    let temperature = Temperature::Fahrenheit(89);
    match_temperature(temperature);

    let temperature = Temperature::Fahrenheit(80);
    match_temperature(temperature);

    // compiler won't take guard conditions into account when checking if all patterns are covered by the match expression
    // - for below match, logically all patterns are covered but we still need the `_` condition
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}

fn match_temperature(temperature: Temperature) {
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }
}
