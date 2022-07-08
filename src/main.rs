mod args;

use std::{path::Path, panic};
use args::RcatArgs;
use clap::Parser;

fn main() {
    let cli = RcatArgs::parse();

    
}

fn read_file(path: &String) -> () {}

/// Check if file exists
fn file_exists(path: &String) -> bool {
    if ! Path::new(path).exists() {
        panic!("'{}': No such file", path)
    }

    true
}

/// Check if path is path of a file
fn is_a_file(path: &String) -> bool {
    if Path::new(path).is_file() {
        panic!("'{}': Is a directory", path)
    }

    true
}