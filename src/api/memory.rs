use super::BackendAPI;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct MemoryAPI;

#[async_trait]
impl BackendAPI for MemoryAPI {
    async fn find(
        &self,
        _: &str,
    ) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        let hash_map = HashMap::new();
        Ok(hash_map)
    }
}
