// A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.
// - Data types can use extra generic type parameters to act as markers or to perform type checking at compile time.
// - These extra parameters hold no storage values, and have no runtime behavior.
#![allow(dead_code)]
pub mod test_case;
use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
// - derive `PartialEq` trait to allow equality test for this type.
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// Storage is allocated for generic type `A`, but not for `B`.
// - therefore, `B` cannot be used in computations.

pub fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // `_tuple1` is PhantomTuple type, specified as `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // `_tuple2` is PhantomTuple type, specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // `_struct1` is PhantomStruct type, specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // `_struct2` is PhantomStruct type, specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
