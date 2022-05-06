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
    server_addr: SocketAddr
}

impl MediaStreamingServer {

    pub fn new(
        video_path: std::path::PathBuf, 
        subtitle_path: Option<std::path::PathBuf>,
        host_ip: String,
    ) -> Self {

        let server_addr: SocketAddr = format!("{}:{}", host_ip, STREAMING_PORT)
            .parse()
            .unwrap_or_else(
                |e| panic!(
                    "Unable to parse socket address for IP={}, error: {}", 
                    host_ip,
                    e
                )
            );

        let video_file = match video_path.exists() {
            true => MediaFile {
                file_path: video_path.clone(),
                host_uri: format!("http://{}", server_addr),
                file_uri: slugify!(video_path.as_path().file_name().unwrap().to_str().unwrap(), separator=".")
            },
            false => {
                panic!("Video file does not exist: {}", video_path.display());
            }
        };

        let subtitle_file = match subtitle_path {
            Some(subtitle_path) => {
                match subtitle_path.exists() {
                    true => Some(MediaFile {
                        file_path: subtitle_path.clone(),
                        host_uri: format!("http://{}", server_addr),
                        file_uri: slugify!(subtitle_path.as_path().file_name().unwrap().to_str().unwrap(), separator=".")
                    }),
                    false => {
                        panic!("Subtitle file does not exist: {}", subtitle_path.display());
                    }
                }
            }
            None => None
        };

        Self {
            video_file,
            subtitle_file,
            server_addr
        }
    }

    pub fn infer_subtitle_from_video(video_path: std::path::PathBuf) -> Option<std::path::PathBuf> {
        let infered_subtitle_path = video_path.with_extension("srt");
        match infered_subtitle_path.exists() {
            true => Some(infered_subtitle_path),
            false => {
                println!(
                    "Tried inferring subtitle file from video file '{}', but it does not exist: {}", 
                    video_path.display(),
                    infered_subtitle_path.display()
                );
                None
            }
        }
    }

    fn get_routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

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

    pub async fn run(&self) {
        let streaming_routes = self.get_routes();
        warp::serve(streaming_routes)
            .run(self.server_addr)
            .await;
    }
}
