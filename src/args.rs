use clap::Parser;

/// my implementation of gnu cat in rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub paths: Vec<String>,
}
