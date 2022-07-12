use natord;
use walkdir::WalkDir;
use walkdir::DirEntry as WDirentry;
use std::path::Path;
use std::fs;
use std::io;
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
    fileentries.sort_by(|a,b| natord::compare(&a.1, &b.1));
    fileentries
}

pub fn accept_and_validate_new_name() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    if buffer.contains('/') {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);
            if !buffer.contains('/') {return buffer}
        }
    }
    buffer
}