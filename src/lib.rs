use walkdir::{self, WalkDir};

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
