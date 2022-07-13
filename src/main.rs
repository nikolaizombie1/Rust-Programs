use plexformatter as pf;
fn main() {
    println!("Input the new name you would the files to be called: ");
    let newname = pf::accept_and_validate_new_name(String::from(""));
    println!("Input the season of which the files belong to: ");
    let season = pf::ask_for_season_and_validate();
    let entries = pf::create_sorted_file_entries(std::env::current_dir().unwrap().as_path());
    println!("Here is a preview of the file changes: ");
    pf::preview_changes(entries.clone(), newname.clone(), season.clone());
    println!("Select a range of files that you would like to edit: ");
    let unfilteredranges =
        pf::create_int_list(pf::accept_and_validate_range_string(entries.clone()));
    if !pf::is_range_ok(entries.clone().len(), unfilteredranges.clone()) {
        eprintln!("Invalid inputed range");
        std::process::exit(1)
    }
    let filtered_entries = pf::create_filtered_entries(entries.clone(), unfilteredranges);
    println!("Here is preview of the filtered files");
    pf::preview_changes(filtered_entries.clone(), newname.clone(), season.clone());
    println!("###################################################");
    println!("#     WARNING: Renaming the files is permanent    #");
    println!("###################################################");
    println!("Would you like to proceed:?[y/n]");
    let reyn = regex::Regex::new(r"([Yy]{1}[Ee]{0,1}[Ss]{0,1})").unwrap();
    let mut yn: String = String::new();
    std::io::stdin()
        .read_line(&mut yn)
        .expect("Error reading stdin");
    if !reyn.is_match(&yn) {
        println!("Script clossed succesfully");
        std::process::exit(0)
    }
    pf::rename_files(filtered_entries.clone(), newname.clone(), season.clone());
    println!(
        "Would you like to move the files to be moved to plex formatted folder structure:?[y/n]"
    );
    let mut yn: String = String::new();
    std::io::stdin()
        .read_line(&mut yn)
        .expect("Error reading stdin");
    if !reyn.is_match(&yn) {
        println!("Script executed succesfully");
        std::process::exit(0)
    }
    pf::create_plex_format_folder_and_move(newname.clone(), season.clone());
    println!("Script executed succesfully");
}
