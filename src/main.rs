use std::{self, env::{self, args}};
use walkdir::{self,WalkDir};
use colored::*;


fn main() {

    let filename = match args().nth(1) {
        Some(i) => i,
        None => {
            println!("{}", 
            "
\nERROR: Missing arguments
Use => path SEARCH_TERM PATH[optional]
            ".red()
        );
            panic!("");
        }
    };

    let path = match args().nth(2) {
        Some(i) => i,
        None => {
            let pathbuf = env::current_dir().unwrap();
            let pathbuf = pathbuf.to_str().unwrap();
            let pathbuf = pathbuf.to_string();
            pathbuf
        }
    };
    
    let path = path.as_str();

    println!("Searching for '{}' in '{}'", filename.bold().bright_blue(), path.bold().bright_green());

    let entry = WalkDir::new(path);
    let entry_two = WalkDir::new(path);

    let matches = match search(entry, filename.as_str()) {
        Some(i) => i,
        None => vec![]
    };
    let almost_matches = match almost_search(entry_two, filename.as_str()) {
        Some(i) => i,
        None => vec![]
    };
    if matches.len() > 0 {
        for m in matches {
            println!("Found a match for '{}' at: {}", filename.bright_blue(), m.bright_green());
        }
    }
    else if almost_matches.len() > 0 {
        for m in almost_matches {
            println!("Found almost a match for '{}' at: {}", filename.bright_blue(), m.bright_green());
        }
    }
    else {
        println!("No matches found for {}", filename.bright_red());
    }
    
}

fn search(entry: WalkDir, filename: &str) -> Option<Vec<String>> {

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
        return None
    }
    Some(matches)
}

fn almost_search(entry: WalkDir, filename: &str) -> Option<Vec<String>> {

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
        return None
    }
    Some(almost_matches)
}
