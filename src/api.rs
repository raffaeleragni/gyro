pub mod jira;
pub mod memory;

use async_trait::async_trait;
use std::{collections::HashMap, error::Error};

#[async_trait]
pub trait BackendAPI {
    async fn find(&self, query: &str) -> Result<HashMap<String, String>, Box<dyn Error>>;
}
