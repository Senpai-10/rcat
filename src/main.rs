mod args;

use args::Args;
use clap::Parser;
use colored::Colorize;
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

        if is_atty() {
            if cli.sep {
                println!("{}", cli.sep_v);
            }

            if cli.print_filename {
                if cli.absolute_path {
                    let pathbuf = std::path::PathBuf::from(path);
                    let abs_path = std::fs::canonicalize(&pathbuf).unwrap();

                    println!("file: {}", abs_path.to_str().unwrap());
                } else {
                    println!("file: {}", path);
                }
            }
        }

        // Only display line numbers if printing to tty
        if cli.numbers && is_atty() {
            let max_lines_number: usize = contents.lines().count();

            for (index, line) in contents.lines().enumerate() {
                let line_number = index + 1;

                println!(
                    "{line_number} {line}",
                    line_number = pad_line_number(&line_number, &max_lines_number),
                    line = line
                );
            }
        } else {
            println!("{}", contents);
        }
    }
}

fn pad_line_number(number: &usize, max_number: &usize) -> String {
    let number_length = get_number_length(number);
    let max_number_length = get_number_length(max_number);

    format!(
        "{}{}",
        " ".repeat(max_number_length - number_length),
        number.to_string().black()
    )
}

fn get_number_length(n: &usize) -> usize {
    n.to_string().chars().count()
}

fn is_atty() -> bool {
    if atty::isnt(atty::Stream::Stdout) {
        return false;
    }

    return true;
}
