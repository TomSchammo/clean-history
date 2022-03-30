use std::{fs, path::PathBuf};

pub fn filter(hist_file: PathBuf) -> Vec<String> {
    let mut filtered = Vec::new();

    if let Ok(buffer) = fs::read(hist_file) {
        let str = String::from_utf8_lossy(&buffer);
        let lines = str.lines();

        for line in lines {
            if !filtered.contains(&line.to_string()) {
                filtered.push(line.to_string());
            }
        }
    }

    filtered
}
