// Annotation of lifetimes in trait methods basically are similar to functions. 
    // - impl may have annotation of lifetimes too.

// A struct with annotation of lifetimes.
#![allow(dead_code)]
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

pub fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}