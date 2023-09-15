use std::fs::{File, OpenOptions};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    model::Item,
    store::{add_item, create_storage, load_store, remove_item, sync_store},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Read data from storage
    Read {
        /// Read from this storage
        storage: String,
        /// Lists all storage contents
        #[arg(short, long)]
        list: bool,
    },

    /// Create an empty storage
    Create { storage: String },

    /// Insert an item in storage
    Insert {
        /// Read from this storage
        storage: String,
        /// The username
        user: String,
        /// The password
        password: String,
        /// The description
        description: String,
    },

    /// Remove an item from storage
    Remove {
        /// Remove from this storage
        storage: String,
        /// The id to remove
        id: usize,
    },
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Read { storage, list } => {
            if list {
                let mut file = File::open(storage)?;
                let storage = load_store(&mut file)?;

                storage.iter().for_each(|(id, item)| {
                    println!("{id}\t{item}");
                });
            }
        }
        Commands::Insert {
            storage,
            user,
            password,
            description,
        } => {
            let mut file = File::open(&storage)?;
            let mut st = load_store(&mut file)?;
            let item = Item::new(
                Some(user.as_str()),
                Some(password.as_str()),
                Some(description.as_str()),
            );
            add_item(&mut st, item)?;
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&storage)?;
            sync_store(&mut st, &mut file)?;
        }
        Commands::Remove { storage, id } => {
            let mut file = File::open(&storage)?;
            let mut st = load_store(&mut file)?;
            remove_item(&mut st, id)?;

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(storage)?;
            sync_store(&mut st, &mut file)?;
        }
        Commands::Create { storage } => {
            let mut file = File::create(storage)?;
            create_storage(&mut file)?;
        }
    };
    Ok(())
}

#[cfg(test)]
mod cli_tests {
    #[test]
    fn it_works() {}
}
