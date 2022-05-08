use std::process::exit;
use log::error;
use pretty_env_logger;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    match crab_dlna::cli::run().await {
        Err(e) => {
            error!("{}", e);
            exit(1);
        },
        Ok(()) => (),
    }
}
