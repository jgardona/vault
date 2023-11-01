mod compression;
mod crypto;

use std::{
    collections::HashMap,
    io::{Read, Write},
    ops::DerefMut,
    usize,
};

use crate::{model::Item, store::crypto::decrypt};
use serde::{Deserialize, Serialize};

use self::{
    compression::{compress, decompress},
    crypto::encrypt,
};

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

pub fn create_storage<W: Write>(writer: &mut W) -> std::io::Result<()> {
    let storage = Store::new();
    let data = serde_json::to_string(&storage)?;
    writer.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add_item<D: DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    item: Item,
) -> std::io::Result<()> {
    let mut id = buffer.len();
    id += 1;
    if buffer.insert(id, item).is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "The key already exists",
        ));
    }
    Ok(())
}

pub fn remove_item<D: DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    id: usize,
) -> std::io::Result<()> {
    if buffer.remove(&id).is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "The key wasn't found",
        ));
    }

    Ok(())
}

pub fn sync_store<W: Write, D: Serialize + DerefMut<Target = HashMap<usize, Item>>>(
    buffer: &mut D,
    writer: &mut W,
) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&buffer)?;
    writer.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn load_store<R: Read>(reader: &mut R) -> std::io::Result<Store> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let data: Store = serde_json::from_str(&buffer)?;
    Ok(data)
}

pub fn lock<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let cipher_data = encrypt(&buffer).expect("cant encrypt given data");
    let mut package = vec![];

    package.extend(cipher_data.2);
    package.extend(cipher_data.1);
    package.extend(cipher_data.0);

    let locked_package = compress(&package)?;
    writer.write_all(&locked_package)?;
    Ok(())
}

pub fn unlock<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    const KEY_SIZE: usize = 32;
    const NONCE_SIZE: usize = 12;

    let mut dec_buffer = &decompress(&buffer)?[..];

    let mut key_buf = [0u8; KEY_SIZE];
    let mut nonce_buf = [0u8; NONCE_SIZE];
    let mut data_buf = Vec::new();

    dec_buffer.read_exact(&mut key_buf)?;
    dec_buffer.read_exact(&mut nonce_buf)?;
    dec_buffer.read_to_end(&mut data_buf)?;

    let decripted_data = decrypt(&data_buf, &nonce_buf, &key_buf).expect("wrong data format");

    writer.write_all(&decripted_data)?;

    Ok(())
}
#[cfg(test)]
mod store_tests {
    use std::io::Cursor;

    use anyhow::{Ok, Result};

    use crate::{
        model::Item,
        store::{add_item, load_store, remove_item},
    };

    use super::{create_storage, lock, sync_store, unlock, Store};

    const EXPECTED: &[u8] = br#"{"data":{"1":{"user":"test@gmail.com","password":"123","description":"just for tests"}}}"#;

    #[test]
    fn it_works() {}

    #[test]
    fn test_lock_unlock() {
        let mut input = Cursor::new(EXPECTED);
        let mut output = Cursor::new(Vec::<u8>::new());
        let mut result_buf = Cursor::new(Vec::<u8>::new());

        lock(&mut input, &mut output).expect("must write output with encoded data");
        output.set_position(0);
        unlock(&mut output, &mut result_buf).expect("must write correct data");
        let result = result_buf.into_inner();
        assert_eq!(EXPECTED, &result);
    }

    #[test]
    fn test_store_workflow() -> Result<()> {
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

        let item2 = Item::new(Some("cris@bol.com.br"), Some("234"), Some("senha do email"));

        add_item(&mut store, item2)?;
        assert_eq!(2, store.len());

        remove_item(&mut store, 2)?;
        assert_eq!(1, store.len());
        Ok(())
    }

    #[test]
    fn test_create_storage() -> Result<()> {
        let expected = r#"{"data":{}}"#;
        let mut cursor = Cursor::new(Vec::<u8>::new());
        create_storage(&mut cursor)?;
        let data = String::from_utf8(cursor.into_inner())?;
        println!("{}", data);
        assert_eq!(expected, data);
        Ok(())
    }

    #[test]
    fn test_store_sync() -> Result<()> {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut store = Store::new();
        let item = Item::new(Some("test@gmail.com"), Some("123"), Some("just for tests"));
        add_item(&mut store, item)?;
        sync_store(&mut store, &mut cursor)?;
        let data = String::from_utf8(cursor.get_ref().to_vec())?;

        assert_eq!(EXPECTED, data.as_bytes());
        cursor.set_position(0);
        let store = load_store(&mut cursor)?;
        let len = store.len();
        assert_eq!(1, len);
        Ok(())
    }
}
