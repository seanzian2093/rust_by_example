pub fn main() {
    // `Iterator::any`, when passed an iterator or called on one, return true if any element satisfies the predicate. Otherwise false.
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`, and closure that passed to `any` captures value
    // - so what is inside `||` should be `&i32`
    // - so use `&x` in `||`, i.e., `|&x|` destructures captured value to `i32` and binds to `x`.
    // - so `x` is `i32`, can be compared with `2`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // - or we can do this way
    println!("2 in vec1: {}", vec1.iter().any(|x| *x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors.

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    // `Iterator::find` is a function which iterates over an iterator and searches for the first value which satisfies some condition.
    // - if none of the values satisfy the condition, it returns None
    // - closure that is passed to `find` captures value by reference

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its items, so we have to destructure `&&i32` to `i32`
    // - closure that is passed to `find` takes reference
    // - similarly we provide `&&x` in `||`, i.e., deconstructuring `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|x| **x == 2));
    // - must use `&&` so below code will not work
    // println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays yields `&i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    let vec = vec![1, 9, 3, 3, 13, 2];

    // Iterator::find gives you a reference to the item. But if you want the index of the item, use Iterator::position.
    // - closure that is passed to `position` capture value

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
    // we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
    // we do not have to destructure
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
