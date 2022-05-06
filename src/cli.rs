use clap::{Args, Parser, Subcommand};

use crate::devices::Render;
use crate::streaming::{MediaStreamingServer, get_serve_ip};
use crate::dlna;

/// A minimal UPnP/DLNA media streamer
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Time in seconds to search and discover streamer hosts
    #[clap(short, long, default_value_t = 5)]
    timeout: u64,

    /// Turn debugging information on
    #[clap(short='b')]
    debug: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and list devices in the network capable of playing media
    List(List),

    // Play a video file
    Play(Play),
}

impl Commands {
    pub async fn run(&self, cli: &Cli) {
        match self {
            Self::List(list) => list.run(cli).await,
            Self::Play(play) => play.run(cli).await,
        }
    }
}

#[derive(Args)]
struct List;

impl List {
    async fn run(&self, cli: &Cli) {
        for render in Render::find_all(cli.timeout).await {
            println!("{}", render);
        }
    }
}

#[derive(Args)]
struct Play {
    /// The hostname or IP to be used to host and serve the files (if not provided we derive it from the local network address)
    #[clap(short='H', long="host")]
    local_host: Option<String>,

    /// Specify the device where to play through a query (scan devices before playing)
    #[clap(short='q', long="query-device")]
    device_query: Option<String>,

    /// Specify the device where to play through its exact location (no scan, faster)
    #[clap(short, long="device")]
    device_url: Option<String>,

    /// The file of the subtitle (if not provided, we derive it from <FILE_VIDEO>)
    #[clap(short, long, parse(from_os_str), value_name="FILE_SUBTITLE")]
    subtitle: Option<std::path::PathBuf>,

    /// Disable subtitles
    #[clap(short, long)]
    no_subtitle: bool,

    /// The file of the video to be played
    #[clap(parse(from_os_str))]
    file_video: std::path::PathBuf,
}

impl Play {
    async fn run(&self, cli: &Cli) {
        let render = self.select_render(cli).await;
        let media_streaming_server = self.build_media_streaming_server(&render).await;
        dlna::play(render, media_streaming_server).await;
    }

    async fn select_render(&self, cli: &Cli) -> Render {
        if let Some(device_url) = &self.device_url {
            println!("Using device: {}", device_url);
            match Render::select_by_url(device_url).await {
                Some(render) => render,
                None => { panic!("Unable to find device at URL: {}", device_url); }
            }
        }
        else if let Some(device_query) = &self.device_query {
            println!("Searching device with query: {}", device_query);
            match Render::select_by_query(cli.timeout, device_query).await {
                Some(render) => render,
                None => { panic!("Unable to find device with query: {}", device_query); }
            }
        }
        else {
            println!("Selecting first available device");
            Render::find_all(cli.timeout)
                .await
                .first()
                .expect("No device found")
                .to_owned()
        }
    }

    async fn build_media_streaming_server(&self, render: &Render) -> MediaStreamingServer {
        let render_host = render.device.url().authority().unwrap().host().to_string();
        let host_ip = self
            .local_host
            .as_ref()
            .unwrap_or(&render_host);

        // FIXME: thread 'tokio-runtime-worker' panicked at 'exact path segments should not be empty'
        MediaStreamingServer::new(
            &self.file_video,
            &self.subtitle,
            &host_ip,
        )
    }
}

pub async fn run() {
    let cli = Cli::parse();
    cli.command.run(&cli).await;
}
