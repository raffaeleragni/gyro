use super::{BackendAPI, Item};
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
pub struct MemoryAPI {
    pub map: HashMap<String, Item>,
}

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[async_trait]
impl BackendAPI for MemoryAPI {
    async fn fetch(&self, key: &str) -> Result<Item, Box<dyn std::error::Error>> {
        self.map
            .get(key)
            .map(|i| Item { key: i.key.clone() })
            .ok_or(Box::new(Error {}))
    }
    async fn find(
        &self,
        query: &str,
    ) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        let q = query.to_lowercase();
        Ok(self
            .map
            .iter()
            .filter(|e| e.0.to_lowercase().contains(&q))
            .map(|i| (i.0.clone(), i.1.key.clone()))
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_map_fetch() -> Result<(), Box<dyn std::error::Error>> {
        let api = MemoryAPI {
            map: HashMap::from([(
                "TEST-1".to_string(),
                Item {
                    key: "TEST-1".to_string(),
                },
            )]),
        };

        let result = api.fetch("TEST-1").await?;
        assert_eq!(result.key, "TEST-1");

        Ok(())
    }

    #[tokio::test]
    async fn test_map_find() -> Result<(), Box<dyn std::error::Error>> {
        let api = MemoryAPI {
            map: HashMap::from([(
                "TEST-1".to_string(),
                Item {
                    key: "TEST-1".to_string(),
                },
            )]),
        };

        assert!(api.find("TEST").await?.contains_key("TEST-1"));

        Ok(())
    }
}
