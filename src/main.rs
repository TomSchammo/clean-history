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
    // FIXME tests don't work yet, because environment variables are a shared resource and the
    // tests run concurrently. So they end up getting in the way of eachother.

    use super::*;

    struct Environment {
        home: String,
        xdg_config_home: String,
    }

    impl Drop for Environment {
        fn drop(&mut self) {
            let h = self.home.clone();
            let x = self.xdg_config_home.clone();

            env::set_var("HOME", h);
            env::set_var("XDG_CONFIG_HOME", x);
        }
    }

    impl Environment {
        fn new() -> Self {
            let home = env::var("HOME");
            let xdg_config_home = env::var("XDG_CONFIG_HOME");

            if home.is_err() || xdg_config_home.is_err() {
                if let Err(e) = home {
                    println!("{}", e)
                };

                if let Err(e) = xdg_config_home {
                    println!("{}", e)
                };
                panic!("Cannot retrieve environment variables");
            }

            Environment {
                home: home.unwrap(),
                xdg_config_home: xdg_config_home.unwrap(),
            }
        }

        fn prepare(home: Option<&str>, xdg_config_home: Option<&str>) {
            env::remove_var("HOME");
            env::remove_var("XDG_CONFIG_HOME");

            if let Some(home) = home {
                env::set_var("HOME", home);
            }

            if let Some(xdg_config_home) = xdg_config_home {
                env::set_var("XDG_CONFIG_HOME", xdg_config_home);
            }
        }
    }

    impl Default for Environment {
        fn default() -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_get_histfile_path_xdg_config_home_set() {
        let _ = Environment::new();
        tests::Environment::prepare(Some("/home/"), Some("/home/user/config"));

        assert_eq!(
            get_histfile_path().to_str().unwrap(),
            "/home/user/config/zsh/histfile"
        );
    }

    #[test]
    fn test_get_histfile_path_home_set() {
        let _ = Environment::new();
        tests::Environment::prepare(Some("/home/user"), None);

        assert_eq!(
            get_histfile_path().to_str().unwrap(),
            "/home/user/.config/zsh/histfile"
        );
    }

    #[test]
    #[should_panic]
    fn test_get_histfile_path_nothing_set() {
        let _ = Environment::new();
        tests::Environment::prepare(None, None);

        let _ = get_histfile_path();
    }
}
