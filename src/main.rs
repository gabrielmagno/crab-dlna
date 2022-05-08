use std::process::exit;
use log::error;

#[tokio::main]
async fn main() {
    match crab_dlna::cli::run().await {
        Err(e) => {
            error!("{}", e);
            exit(1);
        },
        Ok(()) => (),
    }
}
