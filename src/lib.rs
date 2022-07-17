//! This is the library file for the plexformatter crate. It contains the methods used in the main file for plexformatter.
use colored::*;
use natord;
use regex;
use std::{collections::HashSet, fs::{self}, io, path::Path, vec};
use walkdir::{DirEntry as WDirentry, WalkDir};
#[derive(Clone)]
/// Contains the name of the files as String an the extention as String.
pub struct FileEntry {
    name: String,
    extention: String,
}
/// Creates a Vec<FileEntry> given a path.
/// The touple consists of the DirEntry from the walkdir crate in the 0th field, the name of the file in the 1st field and the extention of the file 2nd field.
/// The Vector is sorted by the name of the file in a natural sorted order.
/// The search will only look for files in the current directory, meaning it will not look for files in subdirectories.
/// # Examples
///
/// ```
/// let path = std::env::current_dir().unwrap();
/// let fileentries = plexformatter::create_sorted_file_entries(path.as_path());
/// ```
pub fn create_sorted_file_entries(path: &Path) -> Vec<FileEntry> {
    let entries = WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(|file| file.ok());
    let mut files: Vec<WDirentry> = Vec::new();
    for file in entries {
        if file.metadata().unwrap().is_file() {
            files.push(file);
        }
    }
    let mut fileentries: Vec<FileEntry> = Vec::new();
    for file in files {
        let extention = String::from(
            file.file_name()
                .to_str()
                .unwrap()
                .split('.')
                .last()
                .unwrap(),
        );
        fileentries.push(
            FileEntry {  name: (String::from(file.file_name().to_str().unwrap())), extention: (extention) }
        );
    }
    if fileentries.len() == 0 {
        eprintln!("Directory is empty");
        std::process::exit(1)
    }
    fileentries.sort_by(|a, b| natord::compare(&a.name.to_lowercase(), &b.name.to_lowercase()));
    fileentries
}

/// Validates a name so that it can be a valid file name under linux, i.e does not contain "/" character. 
/// Given a name that does not contain "/" character and is not an empty string, it returns the name without further input.
/// If an invalid name is passed as a parameter it will create a stdin prompt that will then verify the input again.
/// 
/// # Examples
/// 
/// ```
/// let name = String::from("The Simpsons");
/// assert_eq!(
/// String::from("The Simpsons"),plexformatter::accept_and_validate_new_name(name));
/// ```
pub fn accept_and_validate_new_name(name: String) -> String {
    if !name.is_empty() && !name.contains('/') {
        return name;
    }
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading from stdin");
    if buffer.contains('/') {
        loop {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");
            if !buffer.contains('/') {
                return buffer;
            }
        }
    }
    buffer
}

/// The method accepts a Vector<FileEntry> created by the plexformatter::create_sorted_file_entries method as a parameter and it will prompt the user to choose a range of entries from the Vector.
/// Invalid inputs will not be added to the vector and if all entries are invalid it will return an empty vector.
/// Valid formats for selecting the entries are:
/// 1. Number-Number (IE 1-5)
/// 2. Comma seperated values (1,2,3,4,5)
/// 3. Single values seperated by a space (1 2 3 4 5)
/// 
/// # Examples
/// ```
/// let path = std::env::current_dir().unwrap();
/// let fileentries = plexformatter::create_sorted_file_entries(path.as_path());
/// //plexformatter::accept_and_validate_range_string(fileentries); (Added as comment to pass Doctest)
/// ```
pub fn accept_and_validate_range_string(entries: Vec<FileEntry>) -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading from ");
    if buffer.chars().nth(0).unwrap() == '\n' {
        let retstring = format!("1-{}", entries.len());
        return vec![retstring];
    }
    let re = regex::Regex::new(r"^\d+-\d+\d*$|^(\d,)+\d*$|^\d+$").unwrap();
    let ranges: Vec<&str> = buffer.split_ascii_whitespace().collect();
    let ranges = ranges.iter().filter(|range| re.is_match(range));
    let mut retvec: Vec<String> = Vec::new();
    for range in ranges {
        retvec.push(String::from(*range));
    }
    retvec
}

/// Given the Vec<String> created and validated by the plexformatter::accept_and_validate_range_string method and returns a Vec<usize> based on the ranges inputed by the user.
/// 
/// # Examples
/// ```
/// let ranges: Vec<String> = vec![String::from("1-5")];
/// let intvec: Vec<usize> = plexformatter::create_int_vec(ranges);
/// let test: Vec<usize> = vec![1,2,3,4,5];
/// assert_eq!(test,intvec);
/// ```
pub fn create_int_vec(ranges: Vec<String>) -> Vec<usize> {
    let rerange = regex::Regex::new(r"^\d+-\d+\d*$").unwrap();
    let recsv = regex::Regex::new(r"^(\d,)+\d*$").unwrap();
    let resingle = regex::Regex::new(r"^\d+$").unwrap();
    let mut parsedrange: HashSet<usize> = HashSet::new();
    for range in ranges {
        if rerange.is_match(&range) {
            let temprange: Vec<&str> = range.split('-').collect();
            let begin = temprange.get(0).unwrap().parse::<usize>().unwrap();
            let end = temprange.get(1).unwrap().parse::<usize>().unwrap();
            for index in begin..end + 1 {
                parsedrange.insert(index);
            }
        } else if recsv.is_match(&range) {
            let temprange: Vec<&str> = range.split(',').collect();
            for range in temprange {
                parsedrange.insert(range.parse::<usize>().unwrap());
            }
        } else if resingle.is_match(&range) {
            parsedrange.insert(range.parse::<usize>().unwrap());
        }
    }
    let mut parsedrange: Vec<usize> = parsedrange.into_iter().collect();
    parsedrange.sort();
    parsedrange
}

/// Given the length of the vector created by the plexformatter::create_sorted_file_entries method and the vector created by the plexformatter::create_int_vec
/// it will verify that the numbers inside are not less than 1 and not more than the length of the entries vector.
/// 
/// # Examples
/// ```
/// let ranges: Vec<usize> = vec![1,2,3,4,5];
/// let ok_range: bool = plexformatter::is_range_ok(5,ranges);
/// assert!(ok_range);
/// ```
pub fn is_range_ok(len: usize, ranges: Vec<usize>) -> bool {
    for range in ranges {
        if range > len || range < 1 {
            return false;
        }
    }
    true
}

/// Asks the user through stdin a season number in the form of an unsigned integer.
/// If an invalid input is provided, it will prompt the user to input the unsigned integer again.
pub fn ask_for_season_and_validate() -> usize {
    loop {
        let mut season: String = String::new();
        io::stdin()
            .read_line(&mut season)
            .expect("Error reading from stdin");
        let season: usize = match season.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return season;
    }
}



/// Given a vector created by the plexformatter::create_sorted_file_entries 
/// and a vector created by the plexformatter::create_int_vec,
/// it will create a new Vec<FileEntry> that will contain the entries selected by the user defined ranges.
/// # Example
/// ```
/// let path = std::env::current_dir().unwrap();
/// let files = plexformatter::create_sorted_file_entries(path.as_path());
/// let ranges: Vec<usize> = vec![1,2,3,4,5];
/// let filteredvec = plexformatter::create_filtered_entries(files,ranges);
/// ```
pub fn create_filtered_entries(
    entries: Vec<FileEntry>,
    ranges: Vec<usize>,
) -> Vec<FileEntry> {
    let mut retvec: Vec<FileEntry> = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if ranges.contains(&(index + 1)) {
            retvec.push(entry.clone());
        }
    }
    retvec
}

/// Given a vector created by the plexformatter::create_sorted_file_entries or plexformatter:create_filtered_entries,
/// a verified name string from the plexformatter::accept_and_validate_new_name
/// and a verified season from the plexformatter::ask_for_season_and_validate,
/// it will do a dry run of the files by displaying the changes to the user.
pub fn preview_changes(entries: Vec<FileEntry>, newname: String, season: usize) {
    let entries_iter = entries.iter();
    for (index, entry) in entries_iter.enumerate() {
        let output = format!(
            "{}{} {} {} {} {}{}{}{}{}{}",
            (index + 1).to_string().bright_cyan(),
            String::from(".").bright_cyan(),
            entry.name.bright_white(),
            String::from("--->").bright_green(),
            newname.trim_end().bright_red(),
            String::from("S").bright_red(),
            season.to_string().bright_yellow(),
            String::from("E").bright_red(),
            (index + 1).to_string().bright_magenta(),
            String::from(".").bright_white(),
            entry.extention.bright_red()
        );
        println!("{output}");
    }
}

/// Given a vector created by the plexformatter::create_sorted_file_entries or plexformatter:create_filtered_entries,
/// a verified name string from the plexformatter::accept_and_validate_new_name
/// and a verified season from the plexformatter::ask_for_season_and_validate,
/// It will rename the files in the current directory not changing the directory the files are files are located.
pub fn rename_files(entries: Vec<FileEntry>, newname: String, season: usize) {
    let entries_iter = entries.iter();
    for (index, entry) in entries_iter.enumerate() {
        let name = format!(
            "{} S{}E{}.{}",
            newname.trim_end(),
            season,
            index + 1,
            entry.extention
        );
        fs::rename(entry.name.clone(), name).expect("Error renaming files");
    }
}

/// Given the the name created by the plexformatter::accept_and_validate_new_name
/// and the season from plexformatter::ask_for_season_and_validate
/// it will create 2 now folders: The parent folder will have the name of the newname and a subfolder with the name "Season "and the season number appended to it.
pub fn create_plex_format_folder_and_move(newname: String, season: usize) {
    let newfiles = create_sorted_file_entries(std::env::current_dir().unwrap().as_path());
    let path = format!("{}/Season {}/", newname.trim_end(), season);
    fs::create_dir_all(path).expect("Error creating plex directory");
    let path = format!("{}/Season {}/", newname.trim_end(), season);
    for file in newfiles {
        let path_copy = format!("{}{}", &path, file.name);
        fs::rename(file.name, path_copy).expect("Error moving file");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_range_correctly() {
        let inputs: Vec<String> = vec![
            String::from("webfhbewhfbh"),
            String::from("1-5"),
            String::from("ehjfwjbebw1-5jfenwjfe"),
            String::from("rvebvhreb-h43hfh"),
            String::from("_"),
            String::from("6,7,8"),
            String::from("rgejgrjheh,grehbrehgb,grehbhrbe"),
            String::from(",,,,,,,,"),
            String::from("---"),
            String::from("1-1-1"),
            String::from("1aas-b--c"),
        ];
        let correctanswer: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let answer = create_int_vec(inputs);
        assert_eq!(correctanswer, answer);
    }
}
