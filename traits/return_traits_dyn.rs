// The Rust compiler needs to know how much space every function's return type requires.
// - this means all your functions have to return a concrete type.
// - unlike other languages, if you have a trait like Animal, you can't write a function that returns Animal
// - because its different implementations will need different amounts of memory.
// - instead of returning a trait object directly, our functions return a Box which contains some Animal
// - a box is just a reference to some memory in the heap, has a statically-known size, and
// - the compiler can guarantee it points to a heap-allocated Animal, we can return a trait from our function!

// - Rust tries to be as explicit as possible whenever it allocates memory on the heap
// - so if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the dyn keyword, e.g. Box<dyn Animal>.

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// - must use `dyn`
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
