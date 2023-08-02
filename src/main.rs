use clap::Parser;

use crate::{api::jira::Jira, gyro::Gyro};

mod api;
mod gyro;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    #[command(alias = "s")]
    Show,
    #[command(alias = "k")]
    Key { title: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let gyro = Gyro {
        api: Box::new(Jira::new()),
    };
    match args {
        Args::Key { title } => {
            let output = gyro.find_key(&title).await;
            if let Some(output) = output {
                println!("{output}");
            }
        }
        Args::Show => {
            let output = gyro.show().await;
            if let Some(output) = output {
                println!("{}: {}", output.key, output.title);
            }
        }
    }

    Ok(())
}
