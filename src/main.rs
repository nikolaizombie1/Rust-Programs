use std::env;
use std::process;
use plexformatter;
fn main() {
    let files = plexformatter::create_sorted_file_enties(env::current_dir().unwrap_or_else(|e| {
        eprintln!("Error executing in current directory: {}",e);
        process::exit(1)
    }).as_path());
}


