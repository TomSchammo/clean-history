use self::file_operations::filter;
use clap::Parser;
use daemonize::Daemonize;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{thread, time};

#[cfg(not(unix))]
fn main() {
    panic!("This program is only intended to run on Unix systems.");
}

#[cfg(unix)]
mod file_operations;
mod setup;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Launch program as a deamon
    #[clap(short, long, takes_value = false, help = "Launch program as a daemon")]
    daemonize: bool,

    /// If the program is running as a daemon, it runs in a loop.
    /// This is the time between iterations with a default of 1s.
    #[clap(
        short,
        long,
        takes_value = true,
        default_value_t = 1000,
        help = "Time between runs if program is started as a daemon (in ms)"
    )]
    timout: u64,

    /// Provide alternative histfile
    #[clap(
        short = 'H',
        long,
        takes_value = true,
        help = "Provide the path to a history file manually"
    )]
    history: Option<String>,
}

fn start(history_arg: Option<String>) -> PathBuf {
    println!("Starting program...");

    match history_arg {
        Some(arg) => Path::new(&arg).to_path_buf(),
        None => setup::get_histfile_path(),
    }
}

fn main() {
    let args = Args::parse();

    if args.daemonize {
        let exec_name = std::env::current_exe()
            .expect("Can't get the exec path")
            .file_name()
            .expect("Can't get the exec name")
            .to_string_lossy()
            .into_owned();

        let path_base = format!("/tmp/{exec_name}");

        let stdout = File::create(format!("{path_base}.out")).unwrap();
        let stderr = File::create(format!("{path_base}.err")).unwrap();

        let daemonize = Daemonize::new()
            .pid_file(format!("{path_base}.pid"))
            .umask(0o077)
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!("Executed before master process exits"));

        match daemonize.start() {
            Ok(_) => {
                println!("Successfully started daemon!");
                let hist_file = start(args.history);

                loop {
                    filter(hist_file.clone());

                    let timout = time::Duration::from_millis(args.timout);
                    thread::sleep(timout);
                }
            }
            Err(e) => {
                eprintln!("Error when starting deamon!");
                eprintln!("{e}");
            }
        }
    } else {
        let hist_file = start(args.history);
        filter(hist_file);
    }
}
