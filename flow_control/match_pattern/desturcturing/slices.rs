#[allow(dead_code)]
pub fn main() {
    // Try changing the values in the slice
    let slice = &[0, -2, 8][..];
    match_slice(slice);

    let slice = &[1, -2, 8][..];
    match_slice(slice);

    let slice = &[-1, -2, 8][..];
    match_slice(slice);

    let slice = &[3, -2, 8][..];
    match_slice(slice);

    let slice = &[3, -2, 8, 9][..];
    match_slice(slice);

    let slice = &[4, -2, 8, 9][..];
    match_slice(slice);

    let slice = &[][..];
    match_slice(slice);

    // what is &[_] then?
    // let slice = &[_][..];
    // match_slice(slice);
}
fn match_slice(slice: &[i32]) {
    match slice {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!(
            "{:?} : array[0] = 0, array[1] = {}, array[2] = {}",
            slice, second, third
        ),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "{:?} : array[0] = 1, array[2] = {} and array[1] was ignored",
            slice, third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "{:?} : array[0] = -1, array[1] = {} and all the other ones were ignored",
            slice, second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "{:?} : array[0] = 3, array[1] = {} and the other elements were {:?}",
            slice, second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "{:?} : array[0] = {}, middle = {:?}, array[2] = {}",
            slice, first, middle, last
        ),

        &[] => println!("{:?} : Empty slice", slice),
        &[_] => println!("{:?} : What slice is this?", slice),
    }
}

#[cfg(test)]
mod tests {}
