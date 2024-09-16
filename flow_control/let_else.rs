use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    // if `let` binding succeeded then do nothing, except the biding
    //- otherwise execute the `else` block

    // Use Some() for Option
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    // Use Ok for Result
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

pub fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}
