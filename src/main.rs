use clap::{Args, Parser, Subcommand};

mod devices;
mod streaming;
mod dlna;

/// A minimal UPnP/DLNA media streamer
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Time in seconds to search and discover streamer hosts
    #[clap(short, long, default_value_t = 5.0)]
    timeout: f32,

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

#[derive(Args)]
struct List;

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

#[tokio::main]
async fn main() {

    // let cli = Cli::parse();

    // match &cli.command {
    //     Commands::List(_) => {
    //         println!("Listing devices")
    //     }
    //     Commands::Play(..) => {
    //         println!("Playing file")
    //     }
    // }
    
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

    let render = match devices::Render::select_by_query(3, "Kodi").await {
        Some(render) => {
            println!("{}", render);
            render
        }
        None => {
            panic!("No render found");
        }
    };

    let streaming_server = streaming::MediaStreamingServer::new(
        std::path::PathBuf::from("./sample/video.mp4"),
        Some(std::path::PathBuf::from("./sample/video.srt")),
        "127.0.0.1".to_string(),
    );

    dlna::play(render, streaming_server).await;
}
