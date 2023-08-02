pub mod jira;
pub mod memory;

use async_trait::async_trait;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub key: String,
    pub title: String,
}

#[async_trait]
pub trait BackendAPI {
    async fn fetch(&self, key: &str) -> Result<Item, Box<dyn Error>>;
    async fn find(&self, query: &str) -> Result<HashMap<String, String>, Box<dyn Error>>;
}
