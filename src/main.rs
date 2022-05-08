use crab_dlna::Result;

#[tokio::main]
async fn main() -> Result<()> {
    crab_dlna::cli::run().await
}
