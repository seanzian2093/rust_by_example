// If you'd like to wait for a process::Child to finish, you must call Child::wait, which will return a process::ExitStatus
use std::process::Command;

pub fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
