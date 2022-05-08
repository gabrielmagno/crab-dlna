use clap::{Args, Parser, Subcommand};

use crate::{
    devices::{Render, RenderSpec},
    streaming::{MediaStreamingServer, get_serve_ip, infer_subtitle_from_video},
    dlna,
    Result
};

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
    pub async fn run(&self, cli: &Cli) -> Result<()> {
        match self {
            Self::List(list) => list.run(cli).await?,
            Self::Play(play) => play.run(cli).await?,
        }
        Ok(())
    }
}

#[derive(Args)]
struct List;

impl List {
    async fn run(&self, cli: &Cli) -> Result<()> {
        for render in Render::discover(cli.timeout).await? {
            println!("{}", render);
        }
        Ok(())
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
    async fn run(&self, cli: &Cli) -> Result<()> {
        let render = self.select_render(cli).await?;
        let media_streaming_server = self.build_media_streaming_server(&render).await?;
        dlna::play(render, media_streaming_server).await
    }

    async fn select_render(&self, cli: &Cli) -> Result<Render> {
        Render::new(
            if let Some(device_url) = &self.device_url {
                RenderSpec::Location(device_url.to_owned())
            }
            else if let Some(device_query) = &self.device_query {
                RenderSpec::Query(cli.timeout, device_query.to_owned())
            }
            else {
                RenderSpec::First(cli.timeout)
            }
        )
        .await
    }

    async fn build_media_streaming_server(&self, render: &Render) -> Result<MediaStreamingServer> {
        let render_host = render.device.url().authority().unwrap().host().to_string();
        let local_host_ip = get_serve_ip(&render_host).await?;
        let host_ip = self
            .local_host
            .as_ref()
            .unwrap_or(&local_host_ip);

        let subtitle = match &self.no_subtitle {
            false => self.subtitle.clone().or(infer_subtitle_from_video(&self.file_video)),
            true => None
        };

        Ok(
            MediaStreamingServer::new(
                &self.file_video,
                &subtitle,
                &host_ip,
            )?
        )
    }
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    cli.command.run(&cli).await
}
