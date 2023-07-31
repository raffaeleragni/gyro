use crate::{api::jira::Jira, gyro::Gyro};

mod api;
mod gyro;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gyro = Gyro {
        api: Box::new(Jira::new()),
    };
    let output = gyro.gyro_show().await;
    println!("{:#?}", output);
    Ok(())
}
