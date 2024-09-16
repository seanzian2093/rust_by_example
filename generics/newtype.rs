// Years is a tupe struct
struct Years(i64);

pub fn main() {
    let years = Years(42);
    // to get field values from a struct
    // - use tuple or name syntax
    let years_as_primitive_1: i64 = years.0; // Tuple
                                             // - use destructuring
    let Years(years_as_primitive_2) = years; // Destructuring
}
