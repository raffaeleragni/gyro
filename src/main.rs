use std::env;

use crate::api::{jira::Jira, BackendAPI};

mod api;
mod gyro;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = Jira::new();
    let query = String::from(env::args().nth(1).unwrap().as_str());
    let issues = api.find(&query).await?;
    println!("{:#?}", issues);
    Ok(())
}
