use crate::args::Args;
use colored::Colorize;

pub fn cat_file(filename: &String, contents: String, cli: &Args) -> () {
    if is_atty() {
        if cli.sep {
            println!("{}", cli.sep_v);
        }

        if cli.print_filename {
            if cli.absolute_path {
                let pathbuf = std::path::PathBuf::from(filename);
                let abs_path = std::fs::canonicalize(&pathbuf).unwrap();

                println!("File: {}", abs_path.to_str().unwrap().bright_yellow().bold());
            } else {
                println!("File: {}", filename.bright_yellow().bold());
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
