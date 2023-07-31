use crate::api::BackendAPI;

pub struct Gyro {
    pub api: Box<dyn BackendAPI>,
}

impl Gyro {
    pub async fn gyro_show(&self) -> Option<String> {
        let Ok(key) = std::env::var("GYRO_KEY") else { return None; };
        if key.is_empty() {
            return None;
        }
        self.api.fetch(key.as_str()).await.ok().map(|i| i.key)
    }
}

#[cfg(test)]
mod test {
    use crate::api::{memory::MemoryAPI, Item};

    use super::*;

    #[tokio::test]
    async fn called_without_key() {
        std::env::remove_var("GYRO_KEY");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn called_with_empty_key() {
        std::env::set_var("GYRO_KEY", "");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn called_with_not_found_key() {
        std::env::set_var("GYRO_KEY", "BLAH");
        let gyro = Gyro {
            api: Box::<MemoryAPI>::default(),
        };
        let result = gyro.gyro_show().await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn called_with_found_key() {
        std::env::set_var("GYRO_KEY", "TEST-1");
        let mut map = Box::<MemoryAPI>::default();
        map.map.insert(
            "TEST-1".into(),
            Item {
                key: "TEST-1".into(),
            },
        );
        let gyro = Gyro { api: map };
        let result = gyro.gyro_show().await;

        assert_eq!(result, Some("TEST-1".into()));
    }
}
