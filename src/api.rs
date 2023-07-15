mod data;
use self::data::Sections;
use std::{collections::HashMap, env, error::Error};

pub struct Jira {
    host: String,
    user: String,
    token: String,
}

impl Jira {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let host = env::var("JIRA_API_HOST").expect("Required env variable: JIRA_API_HOST");
        let user = env::var("JIRA_API_USER").expect("Required env variable: JIRA_API_USER");
        let token = env::var("JIRA_API_TOKEN").expect("Required env variable: JIRA_API_TOKEN");
        Jira { host, user, token }
    }

    pub async fn find(&self, query: &String) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/rest/api/3/issue/picker?query={}",
                self.host, query
            ))
            .basic_auth(&self.user, Some(&self.token))
            .send()
            .await?
            .text()
            .await?;
        let result = serde_json::from_str::<Sections>(&resp)
            .iter()
            .flat_map(|s| &s.sections)
            .flat_map(|s| &s.issues)
            .map(|i| (i.key.clone(), i.summary_text.clone()))
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_live() {
        let api = Jira::new();
        let query = String::from("test");
        let issues = api.find(&query).await.unwrap();
        println!("{:#?}", issues);
    }
}
