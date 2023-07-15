use std::{collections::HashMap, env};
mod data;
use data::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let host = env::var("JIRA_API_HOST")?;
    let user = env::var("JIRA_USER")?;
    let token = env::var("JIRA_API_TOKEN")?;
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{host}/rest/api/3/issue/picker?query=test"))
        .basic_auth(user, Some(token))
        .send()
        .await?
        .text()
        .await?;
    let resp = serde_json::from_str::<Sections>(&resp);
    let issues: HashMap<&String, &String> = resp
        .iter()
        .flat_map(|s| &s.sections)
        .flat_map(|s| &s.issues)
        .map(|i| (&i.key, &i.summary_text))
        .collect();
    println!("{:#?}", issues);
    Ok(())
}
