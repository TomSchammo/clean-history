use std::{fs, io, path::Path, path::PathBuf};

const PATH_DISPLAY_ERROR: &str = "Cannot display path";
const NEWLINE_BYTES: &[u8] = "\n".as_bytes();

pub fn filter(hist_file: &PathBuf) {
    if let Some(filtered_history) = get_filtered_history(&hist_file.clone()) {
        let filtered_history_bytes = get_filtered_history_bytes(&filtered_history);

        let result = write(&filtered_history_bytes, hist_file);

        if let Err(..) = result {
            match result.unwrap_err() {
                HistFileError::CleanupError => eprintln!("Could not clean up program!"),
                _ => eprintln!("Could not filter history!"),
            }
        } else {
            println!("Successfully filtered history!")
        }
    } else {
        eprintln!("Failed to filter history");
        eprintln!("Could not read history file");
    }
}

/// Filters all the duplicate lines out of the history file
fn get_filtered_history(hist_file: &PathBuf) -> Option<Vec<String>> {
    if let Ok(buffer) = fs::read(hist_file) {
        let str = String::from_utf8_lossy(&buffer);
        let lines = str.lines().collect::<Vec<_>>();

        let mut filtered = Vec::with_capacity(lines.len());

        for line in lines {
            if !filtered.contains(&line.to_string()) {
                filtered.push(line.to_string());
            }
        }
        return Some(filtered);
    }

    None
}

/// Turns the String vector that contains the lines into a byte vector
/// that can be written to a file.
fn get_filtered_history_bytes(history: &Vec<String>) -> Vec<u8> {
    let mut filtered_bytes: Vec<u8> = Vec::new();

    for line in history {
        let mut var = line.as_bytes().to_vec();
        var.append(&mut NEWLINE_BYTES.to_vec());
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
fn write(data: &Vec<u8>, hist_file: &PathBuf) -> Result<(), HistFileError> {
    let temp_file = get_temp_file(&hist_file);

    match fs::rename(hist_file.clone(), temp_file.clone()) {
        Ok(_) => match fs::write(hist_file.clone(), data) {
            Ok(_) => match fs::remove_file(temp_file) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Could not clean up old history file");
                    eprintln!("{e}");
                    Err(HistFileError::CleanupError)
                }
            },
            Err(e) => {
                eprintln!("Cannot write data to history file!");
                eprintln!("{e}");
                println!("Trying to restore old version...");
                if restore(&temp_file, hist_file).is_ok() {
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
            eprintln!(
                "Cannot {} -> {}",
                hist_file.to_str().unwrap_or(PATH_DISPLAY_ERROR),
                temp_file.to_str().unwrap_or(PATH_DISPLAY_ERROR)
            );
            eprintln!("{e}");
            Err(HistFileError::NoWritableTempFile)
        }
    }
}

#[derive(PartialEq, Eq)]
enum HistFileError {
    NoWritableTempFile,
    FailedWrite,
    CleanupError,
}

/// Creates a temporary file that the history file can be copied to,
/// so that the old history can be recovered in case of an error.
fn get_temp_file(hist_file: &Path) -> PathBuf {
    let mut temp_file = hist_file.to_path_buf();

    temp_file.pop();

    if let Some(hist_file_name) = (*hist_file).file_name() {
        if let Some(hist_file_name) = hist_file_name.to_str() {
            temp_file.push(format!("{hist_file_name}.tmp"));
        } else {
            temp_file.push(hist_file_name);
        }
    } else {
        temp_file.push("histfile.tmp");
    }

    temp_file
}

/// Restores the shell history to the state from before the program started
///
/// # Errors
///
/// Any error that can occur when calling `std::fs::rename`.
fn restore(recovery_file: &PathBuf, hist_file: &PathBuf) -> io::Result<()> {
    match fs::rename(recovery_file.clone(), hist_file) {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("Could not recover file!");
            eprintln!(
                "Recovery file is located at '{}'",
                recovery_file.to_str().unwrap_or(PATH_DISPLAY_ERROR)
            );
            Err(e)
        }
    }
}
