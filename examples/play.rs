use std::path::PathBuf;
use crab_dlna::{
    Render,
    RenderSpec,
    MediaStreamingServer,
    STREAMING_PORT_DEFAULT,
    get_local_ip,
    infer_subtitle_from_video,
    Error,
    play,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let discover_timeout_secs = 5;
    let render_spec = RenderSpec::Query(discover_timeout_secs, "Kodi".to_string());
    let render = Render::new(render_spec).await?;
    let host_ip = get_local_ip().await?;
    let host_port = STREAMING_PORT_DEFAULT;
    let video_path = PathBuf::from("/home/crab/Videos/my_video.mp4");
    let inferred_subtitle_path = infer_subtitle_from_video(&video_path);
    let media_streaming_server = MediaStreamingServer::new(
        &video_path,
        &inferred_subtitle_path,
        &host_ip,
        &host_port,
    )?;
    play(render, media_streaming_server).await
}
