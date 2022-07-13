use std::env;
use std::process;
use plexformatter;
fn main() {
    let x = plexformatter::accept_and_validate_range_string();
    println!("{:?}",x)
}


