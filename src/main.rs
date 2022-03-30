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

    let hist_file_result = env::var("HISTFILE");

    if let Ok(path) = hist_file_result {
        hist_file.push(path);
    } else {
        match env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config_home_value) => hist_file.push(xdg_config_home_value),
            Err(_) => {
                eprintln!(
                    "No XDG_CONFIG_HOME environment variable set, falling back to $HOME/.config"
                );

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
    }

    hist_file
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env::VarError;

    struct Environment {
        home: Option<String>,
        xdg_config_home: Option<String>,
        hist_file_path: Option<String>,
    }

    impl Drop for Environment {
        fn drop(&mut self) {
            if let Some(value) = self.home.clone() {
                env::set_var("HOME", value);
            }

            if let Some(value) = self.xdg_config_home.clone() {
                env::set_var("XDG_CONFIG_HOME", value);
            }

            if let Some(value) = self.hist_file_path.clone() {
                env::set_var("HISTFILE", value);
            }
        }
    }

    impl Environment {
        fn new() -> Self {
            let home_path = env::var("HOME");
            let xdg_config_home_path = env::var("XDG_CONFIG_HOME");
            let hist_file_path = env::var("HISTFILE");

            let mut home = None;
            let mut xdg_config_home = None;
            let mut hist_file = None;

            if let Ok(home_path) = home_path {
                home = Some(home_path);
            } else if home_path.err().unwrap() != VarError::NotPresent {
                panic!("Error when retrieving environment variable 'HOME'");
            }

            if let Ok(xdg_config_home_path) = xdg_config_home_path {
                xdg_config_home = Some(xdg_config_home_path);
            } else if xdg_config_home_path.err().unwrap() != VarError::NotPresent {
                panic!("Error when retrieving environment variable 'XDG_CONFIG_HOME'");
            }

            if let Ok(hist_file_path) = hist_file_path {
                hist_file = Some(hist_file_path);
            } else if hist_file_path.err().unwrap() != VarError::NotPresent {
                panic!("Error when retrieving environment variable 'HISTFILE'");
            }

            Environment {
                home,
                xdg_config_home,
                hist_file_path: hist_file,
            }
        }

        fn prepare(
            home: Option<&str>,
            xdg_config_home: Option<&str>,
            hist_file_path: Option<&str>,
        ) {
            env::remove_var("HOME");
            env::remove_var("XDG_CONFIG_HOME");
            env::remove_var("HISTFILE");

            if let Some(home) = home {
                env::set_var("HOME", home);
            }

            if let Some(xdg_config_home) = xdg_config_home {
                env::set_var("XDG_CONFIG_HOME", xdg_config_home);
            }

            if let Some(hist_file_path) = hist_file_path {
                env::set_var("HISTFILE", hist_file_path);
            }
        }
    }

    impl Default for Environment {
        fn default() -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_get_histfile_path_histfile_set() {
        let _ = Environment::new();
        tests::Environment::prepare(
            Some("/home"),
            Some("/home/config"),
            Some("/home/user/.config/zsh/history"),
        );

        assert_eq!(
            get_histfile_path().to_str().unwrap(),
            "/home/user/.config/zsh/history"
        );
    }

    #[test]
    fn test_get_histfile_path_xdg_config_home_set() {
        let _ = Environment::new();
        tests::Environment::prepare(Some("/home/"), Some("/home/user/config"), None);

        assert_eq!(
            get_histfile_path().to_str().unwrap(),
            "/home/user/config/zsh/histfile"
        );
    }

    #[test]
    fn test_get_histfile_path_home_set() {
        let _ = Environment::new();
        tests::Environment::prepare(Some("/home/user"), None, None);

        assert_eq!(
            get_histfile_path().to_str().unwrap(),
            "/home/user/.config/zsh/histfile"
        );
    }

    #[test]
    #[should_panic]
    fn test_get_histfile_path_nothing_set() {
        let _ = Environment::new();
        tests::Environment::prepare(None, None, None);

        let _ = get_histfile_path();
    }
}
