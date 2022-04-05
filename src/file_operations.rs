use std::{
    env::current_exe,
    fs,
    io::{self, Error},
    path::Path,
    path::PathBuf,
};

pub fn filter(hist_file: PathBuf) {
    let filtered_history = get_filtered_history_bytes(get_filtered_history(hist_file.clone()));

    let result = write(filtered_history, hist_file);

    if result.is_err() {
        eprintln!("Could not filter history!");
    }
}

/// Filters all the duplicate lines out of the history file
fn get_filtered_history(hist_file: PathBuf) -> Vec<String> {
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

/// Turns the String vector that contains the lines into a byte vector
/// that can be written to a file.
fn get_filtered_history_bytes(history: Vec<String>) -> Vec<u8> {
    let mut filtered_bytes: Vec<u8> = Vec::new();

    for line in history {
        let mut var = line.as_bytes().to_vec();
        filtered_bytes.append(&mut var);
    }

    filtered_bytes
}

/// Writes all the changes back to the shell history
///
/// # Errors
///
/// - `HistFileError::NoTempFile` if the temporary directory cannot be created to save the history
/// - `HistFileError::NoWritableTempFile` if the history cannot be saved to the temporary directory
/// - `HistFileError::FailedWrite` if there was an error by `std::fs::write` and the new history
/// file has not successfully been created.
///
/// # Panics
///
/// If the new history file cannot be created and the old history file cannot be restored.
fn write(data: Vec<u8>, hist_file: PathBuf) -> Result<(), HistFileError> {
    if let Ok(temp_file) = get_temp_file(&hist_file) {
        match fs::rename(hist_file.clone(), temp_file.clone()) {
            Ok(_) => match fs::write(hist_file.clone(), data) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Cannot write data to history file!");
                    eprintln!("{}", e);
                    println!("Trying to restore old version...");
                    if restore(temp_file, hist_file).is_ok() {
                        println!("Rollback was successful!");
                        Err(HistFileError::FailedWrite)
                    } else {
                        eprintln!("Could not restore old version!");
                        panic!("Panicing!");
                    }
                }
            },
            Err(e) => {
                eprintln!("Cannot save history to temporary directory! Aborting...");
                eprintln!("{}", e);
                Err(HistFileError::NoWritableTempFile)
            }
        }
    } else {
        eprintln!("Could not create temporary file! Aborting...");
        Err(HistFileError::NoTempFile)
    }
}

enum HistFileError {
    NoTempFile,
    NoWritableTempFile,
    FailedWrite,
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
