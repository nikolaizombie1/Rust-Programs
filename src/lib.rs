use natord;
use regex;
use std::{io,fs,path::Path,vec,collections::HashSet};
use walkdir::{WalkDir,DirEntry as WDirentry};
pub fn create_sorted_file_entries(path: &Path) -> Vec<(WDirentry, String, String)> {
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
    let mut fileentries: Vec<(WDirentry, String, String)> = Vec::new();
    for file in files {
        let extention = String::from(
            file.file_name()
                .to_str()
                .unwrap()
                .split('.')
                .last()
                .unwrap(),
        );
        fileentries.push((
            file.clone(),
            String::from(file.file_name().to_str().unwrap()),
            extention,
        ));
    }
    if fileentries.len() == 0 {
        eprintln!("Directory is empty");
        std::process::exit(1)
    }
    fileentries.sort_by(|a, b| natord::compare(&a.1.to_lowercase(), &b.1.to_lowercase()));
    fileentries
}

pub fn accept_and_validate_new_name(name: String) -> String {
    if !name.is_empty() && !name.contains('/') {
        return name
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

pub fn accept_and_validate_range_string(entries: Vec<(WDirentry, String, String)>) -> Vec<String> {
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

pub fn create_int_list(ranges: Vec<String>) -> Vec<usize> {
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

pub fn is_range_ok(len: usize, ranges: Vec<usize>) -> bool {
    for range in ranges {
        if range > len || range < 1 {
            return false;
        }
    }
    true
}

pub fn preview_changes(entries: Vec<(WDirentry, String, String)>,newname: String,season: usize) {
    let entries_iter = entries.iter();
    for (index,entry) in entries_iter.enumerate() {
        println!("{}. {} ----> {newname} S{season}E{}.{}",index+1,entry.1,index+1,entry.2);
    }
}

pub fn ask_for_season_and_validate() -> usize {
    loop {
        let mut season: String = String::new();
        io::stdin()
            .read_line(&mut season)
            .expect("Error reading from ");
        let season: usize = match season.trim().parse() {
           Ok(num) => num,
           Err(_) => continue, 
        };
        return season
    }
}

pub fn rename_files(entries: Vec<(WDirentry, String, String)>,newname: String,season: usize) {
    let entries_iter = entries.iter();
    for (index,entry) in entries_iter.enumerate() {
        let name = format!("{newname} S{season}E{}.{}",index+1,entry.2);
        fs::rename(entry.1.clone(), name).expect("Error renaming files");
    }
}

pub fn create_plex_format_folder_and_move(newname: String, season: usize) {
    let newfiles = create_sorted_file_entries(std::env::current_dir().unwrap().as_path());
    let path = format!("{}/Season {}/",newname,season);
    fs::create_dir_all(path).expect("Error creating plex directory");
    let path = format!("{}/Season {}/",newname,season);
    for file in newfiles {
        let path_copy = format!("{}{}",&path,file.1);
        fs::rename(file.1, path_copy).expect("Error moving file");
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
        let answer = create_int_list(inputs);
        assert_eq!(correctanswer, answer);
    }
}
