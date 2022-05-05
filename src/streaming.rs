use std::net::SocketAddr;
use warp::Filter;

use slugify::slugify;

const STREAMING_PORT: u32 = 9000;

struct MediaFile {
    file_path: std::path::PathBuf,
    host_uri: String,
    file_uri: String
}
pub struct MediaStreamingServer {
    video_file: MediaFile,
    subtitle_file: Option<MediaFile>,
    pub server_addr: SocketAddr
}

impl MediaStreamingServer {

    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

        let video_route = warp::path(self.video_file.file_uri.to_string())
            .and(warp::fs::file(self.video_file.file_path.clone()));

        println!("Video file: {}/{}", self.video_file.host_uri, self.video_file.file_uri);

        let subtitle_route = match &self.subtitle_file {
            Some(subtitle_file) => {
                println!("Subtitle file: {}/{}", subtitle_file.host_uri, subtitle_file.file_uri);
                warp::path(subtitle_file.file_uri.to_string())
                    .and(warp::fs::file(subtitle_file.file_path.clone()))
            }
            None => {
                warp::path("".to_string())
                    .and(warp::fs::file(self.video_file.file_path.clone()))
            }
        };

        return warp::get()
            .and(
                video_route
                .or(subtitle_route)
            );
    }
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
        file_uri: slugify!(video_path.as_path().file_name().unwrap().to_str().unwrap(), separator=".")
    };

    let subtitle_file = match subtitle_path {
        Some(subtitle_path) => {
            Some(MediaFile {
                file_path: subtitle_path.clone(),
                host_uri: format!("http://{}", server_addr),
                file_uri: slugify!(subtitle_path.as_path().file_name().unwrap().to_str().unwrap(), separator=".")
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
