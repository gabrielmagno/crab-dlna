use crate::error::{Error, Result};
use local_ip_address::local_ip;
use log::{debug, info, warn};
use slugify::slugify;
use std::net::SocketAddr;
use warp::Filter;

const STREAMING_PORT: u32 = 9000;

/// A media file to stream
#[derive(Debug, Clone)]
pub struct MediaFile {
    file_path: std::path::PathBuf,
    host_uri: String,
    file_uri: String,
}

impl std::fmt::Display for MediaFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "'{}' @  {}/{}",
            self.file_path.display(),
            self.host_uri,
            self.file_uri,
        )
    }
}

/// A media streaming server
#[derive(Debug, Clone)]
pub struct MediaStreamingServer {
    video_file: MediaFile,
    subtitle_file: Option<MediaFile>,
    server_addr: SocketAddr,
}

impl MediaStreamingServer {
    /// Create a new media streaming server
    pub fn new(
        video_path: &std::path::Path,
        subtitle_path: &Option<std::path::PathBuf>,
        host_ip: &String,
    ) -> Result<Self> {
        let server_addr_str = format!("{}:{}", host_ip, STREAMING_PORT);
        let server_addr: SocketAddr = server_addr_str
            .parse()
            .map_err(|_| Error::StreamingHostParseError(server_addr_str))?;

        debug!("Streaming server address: {}", server_addr);

        debug!("Creating video file route in streaming server");
        let video_file = match video_path.exists() {
            true => MediaFile {
                file_path: video_path.to_path_buf(),
                host_uri: format!("http://{}", server_addr),
                file_uri: slugify!(video_path.display().to_string().as_str(), separator = "."),
            },
            false => {
                return Err(Error::StreamingFileDoesNotExist(
                    video_path.display().to_string(),
                ));
            }
        };

        debug!("Creating subtitle file route in streaming server");
        let subtitle_file = match subtitle_path {
            Some(subtitle_path) => match subtitle_path.exists() {
                true => Some(MediaFile {
                    file_path: subtitle_path.clone(),
                    host_uri: format!("http://{}", server_addr),
                    file_uri: slugify!(
                        subtitle_path.display().to_string().as_str(),
                        separator = "."
                    ),
                }),
                false => {
                    return Err(Error::StreamingFileDoesNotExist(
                        subtitle_path.display().to_string(),
                    ));
                }
            },
            None => None,
        };

        Ok(Self {
            video_file,
            subtitle_file,
            server_addr,
        })
    }

    #[doc(hidden)]
    pub fn video_uri(&self) -> String {
        format!("{}/{}", self.video_file.host_uri, self.video_file.file_uri)
    }

    #[doc(hidden)]
    pub fn video_type(&self) -> String {
        self.video_file
            .file_path
            .as_path()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
    }

    #[doc(hidden)]
    pub fn subtitle_uri(&self) -> Option<String> {
        self.subtitle_file
            .clone()
            .map(|subtitle_file| format!("{}/{}", subtitle_file.host_uri, subtitle_file.file_uri))
    }

    #[doc(hidden)]
    pub fn subtitle_type(&self) -> Option<String> {
        self.subtitle_file.clone().map(|subtitle_file| {
            subtitle_file
                .file_path
                .as_path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string()
        })
    }

    #[allow(clippy::unnecessary_to_owned)]
    fn get_routes(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let video_route = warp::path(self.video_file.file_uri.to_string())
            .and(warp::fs::file(self.video_file.file_path.clone()));

        info!("Video file: {}", self.video_file.file_path.display());
        debug!("Serving video file: {}", self.video_file);

        let subtitle_route = match &self.subtitle_file {
            Some(subtitle_file) => {
                info!("Subtitle file: {}", subtitle_file.file_path.display());
                debug!("Serving subtitle file: {}", subtitle_file);
                warp::path(subtitle_file.file_uri.to_string())
                    .and(warp::fs::file(subtitle_file.file_path.clone()))
            }
            None => {
                info!("No subtitle file");
                warp::path("dummy.srt".to_string())
                    .and(warp::fs::file(self.video_file.file_path.clone()))
            }
        };

        warp::get().and(video_route.or(subtitle_route))
    }

    /// Start the media streaming server.
    pub async fn run(&self) {
        let streaming_routes = self.get_routes();
        warp::serve(streaming_routes).run(self.server_addr).await;
    }
}

/// Identifies the local serve IP address.
pub async fn get_local_ip() -> Result<String> {
    debug!("Identifying local IP address of host");
    Ok(local_ip()
        .map_err(Error::StreamingIdentifyLocalAddressError)?
        .to_string())
}

/// Infer the subtitle file path from the video file path.
pub fn infer_subtitle_from_video(video_path: &std::path::Path) -> Option<std::path::PathBuf> {
    debug!(
        "Inferring subtitle file from video file: {}",
        video_path.display()
    );
    let infered_subtitle_path = video_path.with_extension("srt");
    debug!(
        "Inferred subtitle file: {}",
        infered_subtitle_path.display()
    );
    match infered_subtitle_path.exists() {
        true => Some(infered_subtitle_path),
        false => {
            warn!(
                "Tried inferring subtitle file from video file '{}', but it does not exist: '{}'",
                video_path.display(),
                infered_subtitle_path.display()
            );
            None
        }
    }
}
