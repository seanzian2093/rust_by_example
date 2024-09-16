// A type can implement many different traits. What if two traits both require the same name for a function?
// - e.g., many traits might have a method named get(). They might even have different return types!
// - because each trait implementation gets its own impl block, it's clear which trait's get method you're implementing.
// - when calling those methods, to disambiguate between them, we have to use Fully Qualified Syntax.

trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying "multiple `get` found"
    // - because, after all, there are multiple methods named `get`.
    // println!("{}", form.get());

    // Use fully qualified syntax
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
