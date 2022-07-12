use std::env;
use std::process;
use plexformatter;
fn main() {
    let files = plexformatter::create_sorted_file_entries(env::current_dir().unwrap_or_else(|e| {
        eprintln!("Error executing in current directory: {}",e);
        process::exit(1)
    }).as_path());
    let x = plexformatter::accept_and_validate_new_name();
    println!("{}",x)
}


