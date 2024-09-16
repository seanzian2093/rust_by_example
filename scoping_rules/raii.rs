// RAII, aka Resource Acquisition is Initialization
// The notion of a destructor in Rust is provided through the Drop trait.
// - the destructor is called when the resource goes out of scope.
// - this trait is not required to be implemented for every type
// - only implement it for your type if you require its own destructor logic.

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

pub fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // Customized `Drop` trait
    let x = ToDrop;
    println!("Made a ToDrop!");

    // `_box2` and `x` are destroyed here, and memory gets freed
}
