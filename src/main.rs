use self::file_operations::filter;
use clap::Parser;
use std::path::Path;

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
    /// Launch progress as a deamon
    #[clap(short, long, takes_value = false, help = "Launch program as a deamon")]
    deamonize: bool,

    #[clap(
        short,
        long,
        takes_value = true,
        help = "Provide the path to a history file manually"
    )]
    history: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.deamonize {
        unimplemented!("Start program as a deamon");
    } else {
        let hist_file = match args.history {
            Some(arg) => Path::new(&arg).to_path_buf(),
            None => setup::get_histfile_path(),
        };

        filter(hist_file);
    }
}
