use std::fmt::Debug;
// Expressing bounds without a `where` clause 

// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause

// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}

trait PrintInOption {
    fn print_in_option(self);
}

// in some cases, expressing bounds is impossible without `where` clause
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // - We want `Option<T>: Debug` as our bound because that is what's being printed. 
    // -Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}