use log::error;
use std::process::exit;

#[tokio::main]
async fn main() {
    if let Err(e) = crab_dlna::cli::run().await {
        error!("{}", e);
        exit(1);
    }
}
