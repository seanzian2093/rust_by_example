// A read_lines function can be written in native approach as below
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines_native(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

// Since the method lines() returns an iterator over the lines in the file, we can also
// - perform a map inline and
// - collect the results, yielding a more concise and fluent expression.

fn read_lines_map(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

// A more effiecient approach
// - here we pass ownership of the open File to a BufReader struct
// - BufReader uses an internal buffer to reduce intermediate allocations.
// - update read_lines to return an iterator instead of allocating new String objects in memory for each line.

pub fn main() {
    println!("\nread_line_native()");
    for line in read_lines_native("Cargo.toml") {
        println!("{line}");
    }

    println!("\nread_line_map()");
    for line in read_lines_map("Cargo.toml") {
        println!("{line}");
    }
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./Cargo.toml") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
