use std::{
    collections::HashMap,
    io::{Read, Write},
    ops::DerefMut,
};

use crate::model::Item;
use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    data: HashMap<usize, Item>,
}

impl std::ops::DerefMut for Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl std::ops::Deref for Store {
    type Target = HashMap<usize, Item>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

pub fn create_storage<W: Write>(writer: &mut W) -> Result<()> {
    let storage = Store::new();
    let data = serde_json::to_string(&storage)?;
    writer.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add_item<D: DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    item: Item,
) -> Result<()> {
    let mut id = buffer.len();
    id += 1;
    if buffer.insert(id, item).is_some() {
        return Err(anyhow!("The key {id } already exists"));
    }
    Ok(())
}

// pub fn len<D: DerefMut<Target = HashMap<usize, Item>>>(buffer: &D) -> usize {
//     buffer.len()
// }

pub fn remove_item<D: DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    id: usize,
) -> Result<()> {
    if buffer.remove(&id).is_none() {
        return Err(anyhow!("The key {id} wasn't found"));
    }

    Ok(())
}

pub fn sync_store<W: Write, D: Serialize + DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    writer: &mut W,
) -> Result<()> {
    let serialized = serde_json::to_string(&buffer)?;
    writer.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn load_store<R: Read>(reader: &mut R) -> Result<Store> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let data: Store = serde_json::from_str(&buffer)?;
    Ok(data)
}

#[cfg(test)]
mod store_tests {
    use std::io::Cursor;

    use crate::{
        model::Item,
        store::{add_item, load_store},
    };

    use super::{create_storage, sync_store, Store};

    #[test]
    fn it_works() {}

    #[test]
    fn test_store_workflow() {
        let store = Store::new();
        let mut expected: usize = 0;

        let reference = &*store;
        assert_eq!(expected, reference.len());

        let mut store = Store::new();
        let item1 = Item::new(
            Some("jcbritobr@gmail.com"),
            Some("123"),
            Some("some description"),
        );

        {
            let reference = &mut *store;
            assert_eq!(expected, reference.len());

            assert!(reference.insert(1, item1).is_none());
            expected = 1;
            assert_eq!(expected, reference.len());
        }

        // let item2 = Item::new(Some("cris@bol.com.br"), Some("234"), Some("senha do email"));

        // add_item(&mut store, item2).unwrap();
        // assert_eq!(2, len(&store));

        // remove_item(&mut store, 2).unwrap();
        // assert_eq!(1, len(&store))
    }

    #[test]
    fn test_create_storage() {
        let expected = r#"{"data":{}}"#;
        let mut cursor = Cursor::new(Vec::<u8>::new());
        create_storage(&mut cursor).unwrap();
        let data = String::from_utf8(cursor.into_inner()).unwrap();
        println!("{}", data);
        assert_eq!(expected, data);
    }

    #[test]
    fn test_store_sync() {
        let expected = r#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut store = Store::new();
        let item = Item::new(Some("test@gmail.com"), Some("123"), Some("just for tests"));
        add_item(&mut store, item).unwrap();
        sync_store(&mut store, &mut cursor).unwrap();
        let data = String::from_utf8(cursor.get_ref().to_vec()).unwrap();

        assert_eq!(expected, &data);
        cursor.set_position(0);
        let store = load_store(&mut cursor).unwrap();
        let len = store.len();
        assert_eq!(1, len);
    }
}
