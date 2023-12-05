use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::{de, Serialize};

use crate::json_handler::JsonHandler;

pub trait GetKeyAttributesValue {
    fn get_key_attributes_value(&self) -> String;
}

pub struct Database<'db> {
    path: &'db str,
}

impl<'db> Database<'db> {
    pub fn new(path: &'db str) -> Self {
        if !Path::new(path).exists() {
            let mut file = File::create(path).unwrap();
            file.write_all("[]".as_bytes()).unwrap();
        }

        Self { path }
    }

    pub fn insert<T>(&self, value: T) -> Result<()>
    where
        T: Serialize + de::DeserializeOwned,
    {
        let mut db_content: Vec<T> = JsonHandler::read_from_json(self.path)?;
        db_content.push(value);

        JsonHandler::save_as_json(self.path, &db_content)
    }

    pub fn query<T>(&self, key: &'db str) -> Result<T>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttributesValue,
    {
        let db_content: Vec<T> = JsonHandler::read_from_json(self.path)?;

        if let Some(element) = db_content
            .into_iter()
            .find(|element| element.get_key_attributes_value() == key)
        {
            return Ok(element);
        }

        Err(anyhow!("Element not found"))
    }

    pub fn query_vec<T>(&self, key: &'db str) -> Result<Vec<T>>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttributesValue,
    {
        let db_content: Vec<T> = JsonHandler::read_from_json(self.path)?;

        let result: Vec<T> = db_content
            .into_iter()
            .filter(|element| element.get_key_attributes_value() == key)
            .collect();

        if !result.is_empty() {
            return Ok(result);
        }

        Err(anyhow!("Element not found"))
    }

    pub fn update<T>(&self, key: &'db str, new_element: T) -> Result<()>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttributesValue,
    {
        let mut db_content: Vec<T> = JsonHandler::read_from_json(self.path)?;

        if let Some(element) = db_content
            .iter_mut()
            .find(|element| element.get_key_attributes_value() == key)
        {
            *element = new_element;

            JsonHandler::save_as_json(self.path, &db_content)?;
        }

        Ok(())
    }

    pub fn delete<T>(&self, key: &'db str) -> Result<T>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttributesValue,
    {
        let mut db_content: Vec<T> = JsonHandler::read_from_json(self.path)?;

        if let Some(idx) = db_content
            .iter()
            .position(|element| element.get_key_attributes_value() == key)
        {
            let removed = db_content.swap_remove(idx);

            JsonHandler::save_as_json(self.path, &db_content)?;

            return Ok(removed);
        }

        Err(anyhow!("Element not found"))
    }
}
