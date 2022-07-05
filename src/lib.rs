use walkdir::{self, WalkDir};
use std::{env::{self,args}, path::Path};
use colored::*;

pub fn search(entry: WalkDir, filename: &str) -> Option<Vec<String>> {
    let mut matches = vec![];

    for e in entry {
        let entry = match e {
            Ok(i) => i,
            Err(_) => {
                continue;
            }
        };
        let p = entry.path().to_str().unwrap();
        let file = entry.file_name().to_str().unwrap();
        if file.to_lowercase() == filename.to_lowercase() {
            matches.push(p.to_string().clone());
        }
    }
    if matches.len() == 0 {
        return None;
    }
    Some(matches)
}

pub fn almost_search(entry: WalkDir, filename: &str) -> Option<Vec<String>> {
    let mut almost_matches = vec![];

    for e in entry {
        let entry = match e {
            Ok(i) => i,
            Err(_) => {
                continue;
            }
        };
        let p = entry.path().to_str().unwrap();
        let file = entry.file_name().to_str().unwrap();
        let file = file.to_lowercase();
        if file.contains(filename.to_lowercase().as_str()) {
            almost_matches.push(p.to_string().clone());
        }
    }
    if almost_matches.len() == 0 {
        return None;
    }
    Some(almost_matches)
}

pub fn get_home_dir() -> String {
    let home_dir = match dirs::home_dir() {
        Some(i) => i,
        None => panic!(),
    };

    let home_dir = home_dir.to_str().unwrap();
    let home_dir = home_dir.to_string();

    home_dir
}

pub fn get_file_name() -> String {
    let filename = match args().nth(1) {
        Some(i) => i,
        None => {
            println!(
                "{}",
                "
\nERROR: Missing arguments
Use => path SEARCH_TERM PATH[optional]
            "
                .red()
            );
            panic!("");
        }
    };
    filename
} 

pub fn get_path<'a>(home_dir: &'a str) -> String {
    let path = match args().nth(2) {
        Some(i) => i,
        None => {
            let pathbuf = env::current_dir().unwrap();
            let pathbuf = pathbuf.to_str().unwrap();
            let pathbuf = pathbuf.to_string();
            pathbuf
        }
    };

    let path = path.replace("HOMEDIR", home_dir);

    if !(Path::new(&path.clone()).is_dir()) {
        println!("{}", format!("\nError\nPath doesn't exit: {}\n", &path).red());
        panic!("");
    };

    path
}

