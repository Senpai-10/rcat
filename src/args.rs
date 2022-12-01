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
}
