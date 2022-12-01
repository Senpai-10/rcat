mod args;

use args::Args;
use clap::Parser;
use std::{fs::File, io::Read};

fn main() {
    let cli = Args::parse();

    for path in cli.paths.iter() {
        // TODO: Check if path is not a file

        let mut file = File::open(path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        println!("{}", content);
    }
}

// fn padding_line_number(line_number: &usize, max_lines_number: &usize) -> String {
//     let line_number_length = get_number_length(line_number);
//     let max_lines_number_length = get_number_length(max_lines_number);

//     String::from(" ".repeat(max_lines_number_length - line_number_length))
// }

// fn get_number_length(number: &usize) -> usize {
//     number.to_string().chars().count()
// }
