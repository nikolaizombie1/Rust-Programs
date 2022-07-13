use natord;
use walkdir::WalkDir;
use walkdir::DirEntry as WDirentry;
use std::path::Path;
use std::io;
use regex;
pub fn create_sorted_file_entries(path: &Path) -> Vec<(WDirentry,String)> {
    let entries = WalkDir::new(path).max_depth(1).into_iter().filter_map(|file| file.ok());
    let mut files: Vec<WDirentry> = Vec::new();
    for file in entries {
        if file.metadata().unwrap().is_file() {
            files.push(file);
        }
    }
    let mut fileentries: Vec<(WDirentry,String)> = Vec::new();
    for file in files {
        fileentries.push((file.clone(),String::from(file.file_name().to_str().unwrap())));
    }
    fileentries.sort_by(|a,b| natord::compare(&a.1.to_lowercase(), &b.1.to_lowercase()));
    fileentries
}

pub fn accept_and_validate_new_name() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error reading from stdin");
    if buffer.contains('/') {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Error reading from ");
            if !buffer.contains('/') {return buffer}
        }
    }
    buffer
}

pub fn accept_and_validate_range_string() -> Vec<String> {
    let re = regex::Regex::new(r"^\d+-\d+\d*$|^(\d,)+\d*$|^\d+$").unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error reading from ");
    let ranges: Vec<&str> = buffer.split_ascii_whitespace().collect();
    let ranges = ranges.iter().filter(|range| re.is_match(range));
    let mut retvec: Vec<String> = Vec::new();
    for range in ranges {
        retvec.push(String::from(*range));
    }
    retvec
}