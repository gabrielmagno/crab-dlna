#![warn(missing_docs)]

/*!
crab-dlna is a minimal UPnP/DLNA media streamer.

It allows you to play a local video file in your TV (or any other DLNA compatible device).

This crate provides both a library and a command line tool.

# Features
 - Searching available DLNA devices in the local network
 - Streaming audio
 - Streaming video, with subtitle support

# Example: discover and list devices

crab-dlna provides a function to discover a list devices in the network.

```rust
use crab_dlna::Render;

#[tokio::main]
async fn main() {
    let discover_timeout_secs = 5;
    let renders_discovered = Render::discover(discover_timeout_secs).await.unwrap();
    for render in renders_discovered {
        println!("{}", render);
    }
}
```

# Example: play a video in a render

We can specify a DLNA device render trough a query string,
and then play a certain video in it, automatically detecting
the subtitle file.

```rust,no_run
use std::path::PathBuf;
use crab_dlna::{
    Render,
    RenderSpec,
    MediaStreamingServer,
    get_serve_ip,
    infer_subtitle_from_video,
    Error,
    play,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let discover_timeout_secs = 5;
    let render_spec = RenderSpec::Query(discover_timeout_secs, "Kodi".to_string());
    let render = Render::new(render_spec).await?;
    let host_ip = get_serve_ip(&render.host()).await?;
    let video_path = PathBuf::from("/home/crab/Videos/my_video.mp4");
    let inferred_subtitle_path = infer_subtitle_from_video(&video_path);
    let media_streaming_server = MediaStreamingServer::new(
        &video_path,
        &inferred_subtitle_path,
        &host_ip,
    )?;
    play(render, media_streaming_server).await
}
```

# Technical Details

crab-dlna is basically a one-file DLNA MediaServer and a self DLNA MediaController.

How does `list` work?
1. Issue an SSDP M-Search broadcast message in the network
2. Capture the responses and register the devices
3. Filter only devices that provide [UPnP's AVTransport service](http://www.upnp.org/specs/av/UPnP-av-AVTransport-v3-Service-20101231.pdf)

How does `play` work?
1. Setup an HTTP server to provide the media files to be streamed (including subtitles)
2. Send a `SetAVTransportURI` message to the device, specifying the HTTP URLs of the media files
3. Send a `Play` message to the device
*/

/// Discovery of render devices in the network
mod devices;

/// Streaming of media files
mod streaming;

/// Handling of the DLNA protocol
mod dlna;

/// Command line interface
pub mod cli;

/// Definition of the errors
mod error;

pub use devices::{Render, RenderSpec};
pub use dlna::play;
pub use error::Error;
pub use streaming::{get_serve_ip, infer_subtitle_from_video, MediaStreamingServer};
