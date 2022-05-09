use crab_dlna::Render;

#[tokio::main]
async fn main() {
    let discover_timeout_secs = 5;
    let renders_discovered = Render::discover(discover_timeout_secs).await.unwrap();
    for render in renders_discovered {
        println!("{}", render);
    }
}
