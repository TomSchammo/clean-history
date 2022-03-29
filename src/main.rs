#[cfg(not(unix))]
fn main() {
    panic!("This program is only intended to run on Unix systems.");
}

#[cfg(unix)]
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let hist_file = get_histfile_path();
    println!("{}", hist_file.to_str().unwrap());
}

fn get_histfile_path() -> PathBuf {
    let mut hist_file = PathBuf::new();

    match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home_value) => hist_file.push(xdg_config_home_value),
        Err(_) => {
            eprintln!("No XDG_CONFIG_HOME environment variable set, falling back to $HOME/.config");

            match env::var("HOME") {
                Ok(home_value) => hist_file.push(home_value),
                Err(e) => {
                    eprintln!("No HOME environment variable set, aborting...");
                    panic!("{}", e);
                }
            }
            hist_file.push(".config");
        }
    };

    hist_file.push(Path::new("zsh/histfile"));

    hist_file
}
