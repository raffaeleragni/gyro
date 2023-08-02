use crate::api::BackendAPI;

pub struct Gyro {
    pub api: Box<dyn BackendAPI>,
}

impl Gyro {
    pub async fn gyro_show(&self) -> Option<String> {
        let Ok(key) = std::env::var("GYRO_KEY") else { eprintln!("!no key selected!"); return None; };
        if key.is_empty() {
            eprintln!("!no key selected!");
            return None;
        }
        self.api.fetch(key.as_str()).await.ok().map(|i| i.key)
    }
    pub async fn gyro_find(&self, key: &str) -> Option<String> {
        let Some(map) = self.api.find(key).await.ok() else { return None; };
        if map.len() != 1 {
            return None;
        }
        let result = map.iter().next().unwrap().0.to_owned();
        std::env::set_var("GYRO_KEY", &result);
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    use crate::api::{memory::MemoryAPI, Item};

    use super::*;

    #[tokio::test]
    #[serial]
    async fn called_without_key() {
        std::env::remove_var("GYRO_KEY");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    #[serial]
    async fn called_with_empty_key() {
        std::env::set_var("GYRO_KEY", "");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    #[serial]
    async fn called_with_not_found_key() {
        std::env::set_var("GYRO_KEY", "BLAH");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    #[serial]
    async fn called_with_found_key() {
        std::env::set_var("GYRO_KEY", "TEST-1");
        let mut map = Box::<MemoryAPI>::default();
        map.map.insert(
            "TEST-1".into(),
            Item {
                key: "TEST-1".into(),
                title: "test".into(),
            },
        );
        let gyro = Gyro { api: map };
        let result = gyro.gyro_show().await;

        assert_eq!(result, Some("TEST-1".into()));
    }

    #[tokio::test]
    #[serial]
    async fn find_key() {
        std::env::remove_var("GYRO_KEY");
        let mut map = Box::<MemoryAPI>::default();
        map.map.insert(
            "TEST-1".into(),
            Item {
                key: "TEST-1".into(),
                title: "test".into(),
            },
        );
        let gyro = Gyro { api: map };

        let result = gyro.gyro_find("test").await;
        assert_eq!(result, Some("TEST-1".into()));

        let result = gyro.gyro_show().await;
        assert_eq!(result, Some("TEST-1".into()));
    }

    #[tokio::test]
    #[serial]
    async fn not_found_when_zero() {
        std::env::remove_var("GYRO_KEY");
        let map = Box::<MemoryAPI>::default();
        let gyro = Gyro { api: map };

        let result = gyro.gyro_find("test").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    #[serial]
    async fn not_found_when_too_many() {
        std::env::remove_var("GYRO_KEY");
        let mut map = Box::<MemoryAPI>::default();
        map.map.insert(
            "TEST-1".into(),
            Item {
                key: "TEST-1".into(),
                title: "test".into(),
            },
        );
        map.map.insert(
            "TEST-2".into(),
            Item {
                key: "TEST-2".into(),
                title: "test".into(),
            },
        );
        let gyro = Gyro { api: map };

        let result = gyro.gyro_find("test").await;
        assert_eq!(result, None);
    }
}
