#![allow(dead_code)]
// Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait
    // - For example:
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer and Student
    // - Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

// struct MyProfile{
//     name: String,
//     university: String,
//     fav_languagee: String,
//     git_username: String,
// }
// impl Person for MyProfile {
//     fn name(&self) -> String{
//         self.name
//     }
// }

pub fn main() {}