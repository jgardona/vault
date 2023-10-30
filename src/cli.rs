use std::fs::{File, OpenOptions};

use clap::{Parser, Subcommand};
use tabled::{Table, Tabled};

use crate::{
    model::Item,
    store::{add_item, create_storage, load_store, remove_item, sync_store},
};

#[derive(Tabled)]
struct ItemTable<'a> {
    id: usize,
    user: &'a str,
    password: &'a str,
    description: &'a str,
}

impl<'a> ItemTable<'a> {
    fn new(id: usize, user: &'a str, password: &'a str, description: &'a str) -> Self {
        Self {
            id,
            user,
            password,
            description,
        }
    }
}

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
        #[arg(name = "list", short, long)]
        list: bool,
        /// Gets the size of the storage
        #[arg(name = "size", short, long, conflicts_with = "list")]
        size: bool,
    },

    /// Create an empty storage
    Create {
        /// Read from this storage
        storage: String,
    },

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

pub fn execute() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Read {
            storage,
            list,
            size,
        } => {
            let mut file = File::open(storage)?;
            let storage = load_store(&mut file)?;
            let mut items: Vec<ItemTable> = vec![];

            if list {
                storage.iter().for_each(|(id, item)| {
                    let id = *id;
                    let user = item.user.as_deref().unwrap_or_default();
                    let password = item.password.as_deref().unwrap_or_default();
                    let description = item.description.as_deref().unwrap_or_default();
                    let v = ItemTable::new(id, user, password, description);
                    items.push(v);
                });

                let table = Table::new(items).to_string();
                println!("{table}");
            }

            if size {
                println!("The size of storage is: {}", storage.len());
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
    use std::{fs, path::Path};

    use anyhow::{Ok, Result};
    use assert_cmd::Command;

    const FILE_PATH: &str = "tests/store.json";

    #[test]
    fn it_works() {}

    #[test]
    fn test_create_store() -> Result<()> {
        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("create").arg(FILE_PATH).assert();
        let path = Path::new(FILE_PATH);
        assert!(path.exists());
        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_insert_remove() -> Result<()> {
        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("create").arg(FILE_PATH).assert().success();

        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("insert")
            .arg(FILE_PATH)
            .arg("user1")
            .arg("123456")
            .arg("key one")
            .assert()
            .success();

        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("read")
            .arg(FILE_PATH)
            .arg("-l")
            .assert()
            .stdout(predicates::str::contains("123456"))
            .success();

        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("remove").arg(FILE_PATH).arg("1").assert();

        let mut cmd = Command::cargo_bin("vault")?;
        cmd.arg("read")
            .arg(FILE_PATH)
            .arg("-l")
            .assert()
            .stdout(predicates::boolean::NotPredicate::new(
                predicates::str::diff("123456"),
            ))
            .success();

        let path = Path::new(FILE_PATH);
        fs::remove_file(path)?;
        Ok(())
    }
}
