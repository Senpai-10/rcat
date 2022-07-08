use clap::Parser;

/// rcat my implementation of gnu cat in rust
/// ------------------------------------------
/// Github: https://github.com/senpai-10
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct RcatArgs {
    pub paths: Vec<String>,
}
