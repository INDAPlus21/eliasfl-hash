use clap::Parser;

use eliasfl_hash::db;

#[derive(Debug, Clone, Copy, Parser)]
#[clap(author, version, about)]
struct Cli {
    action: db::Action,
    key: Option<u8>,
    value: Option<u8>,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
