use plexformatter;
fn main() {
    let x = plexformatter::create_sorted_file_entries(std::env::current_dir().unwrap().as_path());
    plexformatter::preview_changes(x, String::from("Fate "), 1)
}
