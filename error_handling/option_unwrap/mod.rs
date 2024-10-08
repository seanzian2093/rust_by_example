pub mod combinator_and_then;
pub mod combinator_map;
pub mod option_default;
pub mod unpack_option;
// An enum called Option<T> in the std library is used when absence is a possibility
// - it manifests itself as one of two "options":
// - Some(T): An element of type T was found
// - None: No element was found
// - two ways of handling
// - `match`, explicitly
// - `unwrap`, implicitly

// The adult has seen it all, and can handle any drink well.
// - all drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

pub fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
