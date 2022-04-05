#[cfg(not(unix))]
fn main() {
    panic!("This program is only intended to run on Unix systems.");
}

#[cfg(unix)]
mod file_operations;
mod setup;

fn main() {
    let hist_file = setup::get_histfile_path();
    // println!("{}", file_operations::get_filtered_history(hist_file).len());
}
