use std::net::SocketAddr;
use warp::Filter;

const STREAMING_PORT: u32 = 9000;

struct MediaFile {
    file_path: std::path::PathBuf,
    host_uri: String,
    file_uri: String
}
pub struct MediaStreamingServer {
    video_file: MediaFile,
    subtitle_file: Option<MediaFile>,
    server_addr: SocketAddr
}

impl MediaStreamingServer {

    pub fn print(&self) {
        println!("Video file: {}/{}", self.video_file.host_uri, self.video_file.file_uri);
    }

    // pub async fn run(&self) {

    //     let video_route = warp::path(self.video_file.file_uri)
    //         .and(warp::fs::file(self.video_file.file_path.clone()));

    //     let subtitle_route = match self.subtitle_file {
    //         Some(subtitle_file) => warp::path(subtitle_file.file_uri).and(warp::fs::file(subtitle_file.file_path.clone())),
    //         None =>                warp::path("dummy".to_string()).and(warp::fs::file("dummy"))
    //     };

    //     // let streaming_server_routes = warp::get().and(video_route.or(subtitle_route));

    //     // println!("Listening on http://{}", self.server_addr);

    //     // warp::serve(streaming_server_routes)
    //     //     .run(self.server_addr)
    //     //     .await;
    // }
}

pub fn create_streaming_server(
        video_path: &std::path::PathBuf, 
        subtitle_path: Option<&std::path::PathBuf>,
        host_ip: String,
    ) -> MediaStreamingServer {

    let server_addr: SocketAddr = format!("{}:{}", host_ip, STREAMING_PORT)
        .parse()
        .expect("Unable to parse socket address");

    let video_file = MediaFile {
        file_path: video_path.clone(),
        host_uri: format!("http://{}", server_addr),
        file_uri: format!(
            "video.{}", 
            video_path.as_path().extension().unwrap().to_str().unwrap()
        )
    };

    let subtitle_file = match subtitle_path {
        Some(subtitle_path) => {
            Some(MediaFile {
                file_path: subtitle_path.clone(),
                host_uri: format!("http://{}", server_addr),
                file_uri: format!(
                    "video.{}", 
                    subtitle_path.as_path().extension().unwrap().to_str().unwrap()
                )
            })
        }
        None => None
    };

    MediaStreamingServer {
        video_file,
        subtitle_file,
        server_addr
    }
}
