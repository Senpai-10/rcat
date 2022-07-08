mod args;

use std::{path::Path, panic, io::Read};
use args::RcatArgs;
use clap::Parser;
use std::fs::File;

fn main() {
    let cli = RcatArgs::parse();

    check_paths(&cli.paths);

    for path in cli.paths.iter() {
        let mut file = File::open(path).unwrap();

        let content = get_file_content(&mut file);

        // max number of lines in the file content
        let max_lines_number: usize = content.lines().count();
        let max_lines_number_length = get_number_length(&max_lines_number);
        let empty_padding = " ".repeat(max_lines_number_length);
        let dash_padding = "─".repeat(max_lines_number_length);
        let path_padding = "─".repeat(path.chars().count());

        println!("{}─┬────────{}┐", dash_padding, path_padding);
        println!("{} │ File: {} │", empty_padding, path);
        println!("{}─┼────────{}┘", dash_padding, path_padding);

        for (index, line) in content.lines().enumerate() {
            let line_number = index + 1;

            println!("{}{} │ {}", padding_space(&line_number, &max_lines_number), line_number, line);
        }
        
        println!("{}─┴────────{}", dash_padding, path_padding);
    }
}

fn padding_space(line_number: &usize, max_lines_number: &usize) -> String {
    let line_number_length = get_number_length(line_number);
    let max_lines_number_length = get_number_length(max_lines_number);

    String::from(" ".repeat(max_lines_number_length - line_number_length))
}

fn get_number_length(number: &usize) -> usize {
    number.to_string().chars().count()
}

fn get_file_content(file: &mut File) -> String {
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content    
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