use clap::Parser;

use eliasfl_hash::db::Database;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
enum Cli {
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

fn main() {
    let args = Cli::parse();

    let db = Database::new();

    let result = match args {
        Cli::Get { key } => db.get(key),
        Cli::Insert { key, value } => db.insert(key, value),
        Cli::Remove { key } => db.remove(key),
    };
    println!("{:#?}", result);
}
