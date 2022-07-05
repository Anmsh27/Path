use colored::*;
use dirs;
use path::*;
use std::{
    self,
    env::{self, args},
    path::Path,
};
use walkdir::{self, WalkDir};

fn main() {
    let home_dir = match dirs::home_dir() {
        Some(i) => i,
        None => panic!(),
    };

    let home_dir = home_dir.to_str().unwrap();

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
    let path = path.as_str();

    if !(Path::new(path.clone()).is_dir()) {
        println!("{}", format!("\nError\nPath doesn't exit: {}\n", &path).red());
        panic!("");
    }

    println!(
        "Searching for '{}' in '{}'",
        filename.bold().bright_blue(),
        path.bold().bright_green()
    );

    let entry = WalkDir::new(path);
    let entry_two = WalkDir::new(path);

    let matches = match search(entry, filename.as_str()) {
        Some(i) => i,
        None => vec![],
    };
    let almost_matches = match almost_search(entry_two, filename.as_str()) {
        Some(i) => i,
        None => vec![],
    };
    if matches.len() > 0 {
        for m in matches {
            println!(
                "Found a match for '{}' at: {}",
                filename.bright_blue(),
                m.bright_green()
            );
        }
    } else {
        println!("No matches found for {}", filename.bright_red());
    }
    if almost_matches.len() > 0 {
        for m in almost_matches {
            println!(
                "Found almost a match for '{}' at: {}",
                filename.bright_blue(),
                m.bright_green()
            );
        }
    }
}
