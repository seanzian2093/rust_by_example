#[allow(dead_code)]
#[derive(Debug)]
struct Foo {
    x: (u32, u32),
    y: u32,
}

pub fn main() {
    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };
    println!("\n{:?}", foo);
    match_struct(foo);

    let foo = Foo { x: (2, 2), y: 3 };
    println!("\n{:?}", foo);
    match_struct(foo);

    // You do not need a match block to destructure structs:
    let faa = Foo { x: (1, 2), y: 3 };
    println!("\n{:?}", faa);
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well:
    #[derive(Debug)]
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    println!("\n{:?}", bar);
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}

fn match_struct(foo: Foo) {
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
