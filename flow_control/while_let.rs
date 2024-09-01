
pub fn main() {
    while_let(7);
    while_let(10);
    
    println!();
    loop_match(7);
    loop_match(10);
}

fn while_let(arg: i32) {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(arg);
    
    // This reads: "while `let` destructures `optional` into `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // - Less rightward drift and doesn't require explicitly handling the failing case.
    }
    // `if let` had additional optional `else`/`else if` clauses. `while let` does not have these.
}

// while_let() equivalents to loop_match()
fn loop_match(arg: i32) {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(arg);

    // Repeatedly try this test.
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            },
            // Use explicit `break` to quit the loop when the destructure fails:
            _ => { break; }
        }
    }

}