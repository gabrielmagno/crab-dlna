mod devices;
mod streaming;
mod dlna;
mod cli;

#[tokio::main]
async fn main() {

    crate::cli::run().await;
    
    // for render in devices::Render::find_all(5).await {
    //     println!("{}", render);
    // }

    // let render = match devices::Render::select_by_url("http://172.22.176.1:1313/").await {
    //     Some(render) => {
    //         println!("{}", render);
    //         render
    //     }
    //     None => {
    //         panic!("No render found");
    //     }
    // };

    // let render = match devices::Render::select_by_query(3, "Kodi").await {
    //     Some(render) => {
    //         println!("{}", render);
    //         render
    //     }
    //     None => {
    //         panic!("No render found");
    //     }
    // };

    // let streaming_server = streaming::MediaStreamingServer::new(
    //     std::path::PathBuf::from("./sample/video.mp4"),
    //     Some(std::path::PathBuf::from("./sample/video.srt")),
    //     "127.0.0.1".to_string(),
    // );

    // dlna::play(render, streaming_server).await;
}
