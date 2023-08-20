# crab-dlna
[![CICD](https://github.com/gabrielmagno/crab-dlna/actions/workflows/CICD.yml/badge.svg)](https://github.com/gabrielmagno/crab-dlna/actions/workflows/CICD.yml)
[![Version info](https://img.shields.io/crates/v/crab-dlna.svg)](https://crates.io/crates/crab-dlna)

crab-dlna is a minimal UPnP/DLNA media streamer, available both as a standlone CLI (command line interface) application and a Rust library.

It allows you to play a local video file in your TV (or any other DLNA compatible device).

## Features
 - Searching available DLNA devices in the local network
 - Streaming audio
 - Streaming video, with subtitle support

## Installation

In the GitHub Releases of this repository we provide [archives of precompiled binaries](https://github.com/gabrielmagno/crab-dlna/releases) of crab-dlna, available for **Linux**, **Windows**, and **macOS**.

### cargo

Installation via cargo is done by installing the `crab-dlna` crate:
```bash
# If required, update Rust on the stable channel
rustup update stable

cargo install crab-dlna

# Alternatively, --locked may be required due to how cargo install works
cargo install crab-dlna --locked
```

## Usage (CLI)

You can list all the CLI commands by running:
```
crab-dlna --help
```

### List

Scan compatible devices and list the available ones:
```bash
crab-dlna list
```

If your device is not being listed, you might need to increase the search timeout:
```bash
crab-dlna -t 20 list
```

### Play

Play a video, automatically loading the subtitles if available, selecting a random device:
```bash
crab-dlna play That.Movie.mkv
```

Play a video, specifying the device through query (scan devices before playing):
```bash
crab-dlna play That.Movie.mkv -q "osmc"
```

Play a video, specifying the device through its exact location (no scan, faster):
```bash
crab-dlna play That.Movie.mkv -d "http://192.168.1.13:1082/"
```

## Usage (library)

Add `crab-dlna` and `tokio` to your dependencies:
```toml
[dependencies] 
tokio = { version = "1", features = ["full"] }
crab-dlna = "0.2"
```

### Example: discover and list devices

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

### Example: play a video in a device

We can specify a DLNA device render trough a query string, 
and then play a certain video in it, automatically detecting 
the subtitle file.

```rust
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
```

You can access the full [documentation](https://docs.rs/crab-dlna/) to see more details about the library.

## License

Copyright (c) 2022 Gabriel Magno.

`crab-dlna` is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
