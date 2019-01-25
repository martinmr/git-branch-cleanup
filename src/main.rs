#[macro_use]

extern crate clap;
extern crate git2;

mod flags;
mod git_cleanup;

fn main() {
    let options = flags::parse_flags().unwrap();
    match git_cleanup::run_cleanup(&options) {
        Ok(_) => return,
        Err(err) => panic!("Exited with error: {}", err),
    };
}
