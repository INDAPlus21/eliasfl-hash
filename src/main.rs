use std::path::PathBuf;

use clap::Parser;

use eliasfl_hash::db::Database;

#[derive(Debug, Clone, Parser)]
enum Subcommand {
    /// Get value by key
    Get {
        /// The key to get
        key: u64,
    },
    /// Insert value with key
    Insert {
        /// The key to insert at
        key: u64,
        /// The value to insert
        value: String,
    },
    /// Remove value with key
    Remove {
        /// The key to remove
        key: u64,
    },
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Cli {
    /// The subcommand to execute on the database
    #[clap(subcommand)]
    subcommand: Subcommand,
    /// The database file to read
    #[clap(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let db = Database::new(args.file);

    let result = match args.subcommand {
        Subcommand::Get { key } => db.get(key),
        Subcommand::Insert { key, value } => db.insert(key, value),
        Subcommand::Remove { key } => db.remove(key),
    };
    println!("{:#?}", result);
}
