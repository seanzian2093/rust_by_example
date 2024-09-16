/*
If you have implemented the From trait for your type, Into will call it when necessary. Note, however, that the converse is not true:
implementing Into for your type will not automatically provide it with an implementation of From. */
#![allow(unused_variables)]
use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Into is automatically implemented as following
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

pub fn main() {
    // from
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // into
    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
