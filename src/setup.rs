use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn get_histfile_path() -> PathBuf {
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
                        panic!("{e}");
                    }
                }
                hist_file.push(".config");
            }
        };

        hist_file.push(Path::new("zsh/histfile"));
    }

    hist_file
}

/// Looks for the config in $XDG_CONFIG_HOME/clean-history/config.json.
/// If $XDG_CONFIG_HOME is not set, it defaults to $HOME/.config.
/// If $HOME is not set function aborts.
fn get_config_path() -> Option<PathBuf> {
    let mut config_file = PathBuf::new();

    match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home_value) => config_file.push(xdg_config_home_value),
        Err(_) => {
            eprintln!("No XDG_CONFIG_HOME environment variable set, falling back to $HOME/.config");

            match env::var("HOME") {
                Ok(home_value) => config_file.push(home_value),
                Err(e) => {
                    eprintln!("No HOME environment variable set, aborting...");
                    eprintln!("{e}");
                    return None;
                }
            }
            config_file.push(".config");
        }
    }

    config_file.push("clean-history/config.json");

    Some(config_file)
}

/// Reads the data in config file.
/// The read data will then be parsed, wrapped in a
/// `Config` and returned.
fn get_config(config_path: PathBuf) -> Option<Config> {
    let file_contents = fs::read_to_string(config_path).ok();

    if let Some(file_contents) = file_contents {
        return match json::parse(&file_contents) {
            Ok(val) => Some(parse_json_object(val)),
            Err(_) => None,
        };
    }
    None
}

/// Parses the json value returned by `json::parse`.
/// This function will look for the memebers of `Config`.
///
/// If a member is not found or defined incorrectly,
/// it will simply be set to `None`.
fn parse_json_object(object: json::JsonValue) -> Config {
    let histfile: Option<PathBuf> = {
        if object["histfile"].is_null() {
            None
        } else {
            object["histfile"].as_str().map(PathBuf::from)
        }
    };

    let blacklist: Option<Vec<String>> = {
        if object["blacklist"].is_null() {
            // 'blacklist' is not set in config
            None
        } else if object["blacklist"].is_array() {
            let vals = object["blacklist"]
                .members()
                .filter(|val| val.as_str().is_some())
                .map(|val| val.to_string())
                .collect();
            Some(vals)
        } else {
            // 'blacklist' is not an array, so set incorrectly
            None
        }
    };

    let max_char_limit: Option<u64> = {
        if object["max_char_limit"].is_null() {
            None
        } else {
            // 'max_char_limit' is either returned, or not set correctly
            object["max_char_limit"].as_u64()
        }
    };

    let min_char_limit: Option<u64> = {
        if object["min_char_limit"].is_null() {
            None
        } else {
            // 'min_char_limit' is either returned, or not set correctly
            object["min_char_limit"].as_u64()
        }
    };

    Config {
        histfile,
        blacklist,
        max_char_limit,
        min_char_limit,
    }
}

pub struct Config {
    /// Location of the histfile.
    /// This value is used if provided.
    histfile: Option<PathBuf>,

    /// Commands that will automatically be removed from the history
    blacklist: Option<Vec<String>>,

    /// Commands longer than 'max_char_limit' will automatically be removed from the history
    max_char_limit: Option<u64>,

    /// Commands shorter than 'min_char_limit' will automatically be removed from the history
    min_char_limit: Option<u64>,
}

/// Reads the config file and parses it into a `Config`.
pub fn config() -> Config {
    if let Some(path) = get_config_path() {
        if let Some(config) = get_config(path) {
            config
        } else {
            todo!("What to do when config could not be retrieved?")
        }
    } else {
        todo!("What to do when config could not be found?")
    }
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
            if let Some(value) = &self.home {
                env::set_var("HOME", value);
            }

            if let Some(value) = &self.xdg_config_home {
                env::set_var("XDG_CONFIG_HOME", value);
            }

            if let Some(value) = &self.hist_file_path {
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
