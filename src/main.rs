use plexformatter;
use std::env;
fn main() {
    let x = plexformatter::create_sorted_file_entries(env::current_dir().unwrap().as_path());
    println!("{:?}",x);
    let y = plexformatter::accept_and_validate_range_string(x);
    println!("{:?}",y);
}


