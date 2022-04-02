use std::{
    env::current_exe,
    fs,
    io::{self, Error},
    path::Path,
    path::PathBuf,
};

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

/// Creates a temporary directory that the history file can be moved to,
/// so that the old history can be recovered in case of an error.
///
/// # Errors
///
/// Any error that can occur when calling `std::fs::create_dir_all`.
fn get_temp_file(hist_file: &Path) -> Result<PathBuf, Error> {
    let mut temp_file = PathBuf::new();

    temp_file.push("/tmp");

    if let Some(current_exe) = current_exe()
        .unwrap_or_else(|_| Path::new("clean-histoy").to_path_buf())
        .file_name()
    {
        temp_file.push(current_exe);
    }

    if !temp_file.exists() {
        match fs::create_dir_all(temp_file.clone()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "Could not create '{}'",
                    temp_file.to_str().unwrap_or("Cannot display path")
                );
                return Err(e);
            }
        }
    }

    if let Some(hist_file_name) = (*hist_file).file_name() {
        if let Some(hist_file_name) = hist_file_name.to_str() {
            temp_file.push(format!("{}.tmp", hist_file_name));
        } else {
            temp_file.push(hist_file_name);
        }
    } else {
        temp_file.push("histfile.tmp");
    }

    Ok(temp_file)
}

/// Restores the shell history to the state from before the program started
///
/// # Errors
///
/// Any error that can occur when calling `std::fs::rename`.
fn restore(recovery_file: PathBuf, hist_file: PathBuf) -> io::Result<()> {
    match fs::rename(recovery_file.clone(), hist_file) {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("Could not recover file!");
            eprintln!(
                "Recovery file is located at '{}'",
                recovery_file.to_str().unwrap_or("Cannot display path")
            );
            Err(e)
        }
    }
}
