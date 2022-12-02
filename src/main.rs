mod args;
mod cat_file;

use args::Args;
use cat_file::cat_file;
use clap::Parser;
use std::{fs::File, io::Read};

fn main() {
    let cli = Args::parse();

    for path in cli.paths.iter() {
        let mut file = match File::open(path) {
            Ok(c) => c,
            Err(e) => {
                if cli.skip {
                    continue;
                }

                println!("'{}' {}", path, e);
                continue;
            }
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Err(e) => {
                if cli.skip {
                    continue;
                }

                println!("Error: '{}' {}", path, e.kind());
                continue;
            }
            _ => {}
        };

        cat_file(path, contents, &cli);
    }
}

