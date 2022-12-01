mod args;

use args::Args;
use clap::Parser;
use std::{fs::File, io::Read};

fn main() {
    let cli = Args::parse();

    for path in cli.paths.iter() {
        // TODO: Check if path is not a file

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        if cli.numbers {
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
        number
    )
}

fn get_number_length(n: &usize) -> usize {
    n.to_string().chars().count()
}
