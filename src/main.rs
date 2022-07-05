use colored::*;
use path::*;
use std::{
    self,
    env::args,
    fs
};
use walkdir::{self, WalkDir};

fn main() {
    let home_dir = get_home_dir();
    let home_dir = home_dir.as_str();

    let filename = get_file_name();

    let path = get_path(home_dir);
    let path = path.as_str();

    let output_file_name = match args().nth(3) {
        Some(i) => i,
        None => "output.txt".to_string(),
    };
    
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

    let mut contents = "".to_string();
    if matches.len() > 0 {
        for m in matches {
            println!(
                "Found a match for '{}' at: {}",
                filename.bright_blue(),
                m.bright_green()
            );
            if !&output_file_name.is_empty() {
                let n = format!("\nFound a match for '{}' at: {}\n", filename, m);
                contents += n.as_str();
            }
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

            if !&output_file_name.is_empty() {
                let n = format!("\nFound a almost a match for '{}' at: {}\n", filename, m);
                contents += n.as_str();
            }
        }
    }
    fs::write(output_file_name, contents)
        .unwrap_or_else(|_| println!("{}", "Error in writing to file".red()));
}
