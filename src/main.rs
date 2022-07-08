mod args;

use std::{path::Path, panic};
use args::RcatArgs;
use clap::Parser;
use std::fs::File;

fn main() {
    let cli = RcatArgs::parse();

    check_paths(&cli.paths);

    for path in cli.paths.iter() {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        match File::open(path).and_then(|mut file| std::io::copy(&mut file, &mut handle)) {
            Err(err) => eprintln!("{}", err),
            _ => (),
        }
    }
}

/// Check if file exists
fn file_exists(path: &String) -> bool {
    if !Path::new(path).exists() {
        panic!("'{}': No such file or directory", path)
    }

    true
}

/// Check if path is path of a file
fn is_a_file(path: &String) -> bool {
    if ! Path::new(path).is_file() {
        panic!("'{}': Is a directory", path)
    }

    true
}

/// Check if paths is not a file or doesn't exists
/// before reading them.
fn check_paths(paths: &Vec<String>) -> () {
    for path in paths.iter() {
        file_exists(&path);
        is_a_file(&path);
    }
}