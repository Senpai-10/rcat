use clap::Parser;

/// my implementation of gnu cat in rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub paths: Vec<String>,

    /// Show line numbers
    #[clap(short, long)]
    pub numbers: bool,

    /// Ignore path if not found or is a directory
    #[clap(short, long)]
    pub skip: bool,

    /// Print filename before printing contents
    #[clap(short, long)]
    pub print_filename: bool,

    /// Print filename as absolute path
    #[clap(short, long)]
    pub print_absolute_path: bool,

    /// Add a separater between files
    #[clap(short, long)]
    pub sep: bool,

    /// Separater to use between files
    #[clap(short, long)]
    pub sep_v: Option<String>,
}
