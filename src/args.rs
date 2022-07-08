use clap::Parser;

/// my implementation of gnu cat in rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct RcatArgs {
    pub paths: Vec<String>,

    #[clap(short, long, help = "number all output lines")]
    pub number: bool
}
