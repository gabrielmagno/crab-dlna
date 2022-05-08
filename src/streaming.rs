use std::net::{SocketAddr, UdpSocket};
use warp::Filter;
use crate::{Error, Result};

use slugify::slugify;

const DUMMY_PORT: u32 = 0;
const STREAMING_PORT: u32 = 9000;

#[derive(Debug, Clone)]
struct MediaFile {
    file_path: std::path::PathBuf,
    host_uri: String,
    file_uri: String
}

#[derive(Debug, Clone)]
pub struct MediaStreamingServer {
    video_file: MediaFile,
    subtitle_file: Option<MediaFile>,
    server_addr: SocketAddr
}

impl MediaStreamingServer {

    pub fn new(
        video_path: &std::path::PathBuf, 
        subtitle_path: &Option<std::path::PathBuf>,
        host_ip: &String,
    ) -> Result<Self> {

        let server_addr: SocketAddr = format!("{}:{}", host_ip, STREAMING_PORT)
            .parse()
            .map_err(|_| Error::StreamingHostParseError(host_ip.to_owned()))?;

        let video_file = match video_path.exists() {
            true => MediaFile {
                file_path: video_path.clone(),
                host_uri: format!("http://{}", server_addr),
                file_uri: slugify!(video_path.display().to_string().as_str(), separator=".")
            },
            false => {
                return Err(Error::StreamingFileDoesNotExist(video_path.display().to_string()));
            }
        };

        let subtitle_file = match subtitle_path {
            Some(subtitle_path) => {
                match subtitle_path.exists() {
                    true => Some(MediaFile {
                        file_path: subtitle_path.clone(),
                        host_uri: format!("http://{}", server_addr),
                        file_uri: slugify!(subtitle_path.display().to_string().as_str(), separator=".")
                    }),
                    false => {
                        return Err(Error::StreamingFileDoesNotExist(subtitle_path.display().to_string()));
                    }
                }
            }
            None => None
        };

        Ok(Self{video_file, subtitle_file, server_addr})
    }

    pub fn video_uri(&self) -> String {
        format!("{}/{}", self.video_file.host_uri, self.video_file.file_uri)
    }

    pub fn video_type(&self) -> String {
        self.video_file.file_path.as_path().extension()
            .unwrap_or_default().to_str().unwrap_or_default().to_string()
    }

    pub fn subtitle_uri(&self) -> Option<String> {
        self.subtitle_file.clone().map(
            |subtitle_file| 
            format!("{}/{}", subtitle_file.host_uri, subtitle_file.file_uri)
        )
    }

    pub fn subtitle_type(&self) -> Option<String> {
        self.subtitle_file.clone().map(
            |subtitle_file| 
            subtitle_file.file_path.as_path().extension()
                .unwrap_or_default().to_str().unwrap_or_default().to_string()
        )
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
                warp::path("dummy.srt".to_string())
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

pub async fn get_serve_ip(target_host: &String) -> Result<String> {
    let target_addr: SocketAddr = format!("{}:{}", target_host, DUMMY_PORT)
        .parse()
        .map_err(|_| Error::StreamingHostParseError(target_host.to_owned()))?;

    Ok(
        UdpSocket::bind(target_addr)
            .map_err(|err| Error::StreamingRemoteRenderConnectFail(target_addr.to_string(), err))?
            .local_addr()
            .map_err(|err| Error::StreamingIdentifyLocalAddressError(err))?
            .ip()
            .to_string()
    )
}

pub fn infer_subtitle_from_video(video_path: &std::path::PathBuf) -> Option<std::path::PathBuf> {
    let infered_subtitle_path = video_path.with_extension("srt");
    match infered_subtitle_path.exists() {
        true => Some(infered_subtitle_path),
        false => {
            println!(
                "Tried inferring subtitle file from video file '{}', but it does not exist: '{}'", 
                video_path.display(),
                infered_subtitle_path.display()
            );
            None
        }
    }
}

