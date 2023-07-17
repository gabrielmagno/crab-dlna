use crate::devices::RenderSpec;
use std::fmt;

/// Errors that can happen inside crab-dlna
#[derive(Debug)]
pub enum Error {
    /// An error occurred while discovering devices
    DevicesDiscoverFail(rupnp::Error),
    /// An error occurred while iterating over discovered devices
    DevicesNextDeviceError(rupnp::Error),
    /// An error occurred while parsing a device URL
    DevicesUrlParseError(String),
    /// An error occurred while parsing and creating a device
    DevicesCreateError(String, rupnp::Error),
    /// An error occurred when the specified render is not found
    DevicesRenderNotFound(RenderSpec),
    /// An error occurred parsing a host or IP address
    StreamingHostParseError(String),
    /// An error occurred when a certain media file does not exist
    StreamingFileDoesNotExist(String),
    /// An error occurred while trying to connect to the render
    StreamingRemoteRenderConnectFail(String, std::io::Error),
    /// An error occurred while trying to identify the host IP address
    StreamingIdentifyLocalAddressError(local_ip_address::Error),
    /// An error occurred while sending the SetAVTransportURI DLNA action to the render
    DLNASetAVTransportURIError(rupnp::Error),
    /// An error occurred while sending the Play DLNA action to the render
    DLNAPlayError(rupnp::Error),
    /// An error occurred while serving and streaming the media files
    DLNAStreamingError(tokio::task::JoinError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DevicesDiscoverFail(err) => write!(f, "Failed to discover devices: {}", err),
            Error::DevicesNextDeviceError(err) => write!(f, "Failed to get next device: {}", err),
            Error::DevicesUrlParseError(url) => write!(f, "Failed to parse URL '{}'", url),
            Error::DevicesCreateError(url, err) => write!(
                f,
                "Failed to parse and create device from '{}': {}",
                url, err
            ),
            Error::DevicesRenderNotFound(render_spec) => match render_spec {
                RenderSpec::Location(device_url) => {
                    write!(f, "No render found at '{}'", device_url)
                }
                RenderSpec::Query(timeout, device_query) => write!(
                    f,
                    "No render found withing {} seconds with query '{}'",
                    timeout, device_query
                ),
                RenderSpec::First(timeout) => {
                    write!(f, "No render found within {} seconds", timeout)
                }
            },
            Error::StreamingHostParseError(addr) => {
                write!(f, "Failed to parse host address '{}'", addr)
            }
            Error::StreamingFileDoesNotExist(file) => write!(f, "File '{}' does not exist", file),
            Error::StreamingRemoteRenderConnectFail(host, err) => {
                write!(f, "Failed to connect to remote render '{}': {}", host, err)
            }
            Error::StreamingIdentifyLocalAddressError(err) => {
                write!(f, "Failed to identify local address: {}", err)
            }
            Error::DLNASetAVTransportURIError(err) => {
                write!(f, "Failed to set AVTransportURI: {}", err)
            }
            Error::DLNAPlayError(err) => write!(f, "Failed to Play: {}", err),
            Error::DLNAStreamingError(err) => write!(f, "Failed to stream: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DevicesDiscoverFail(err) => Some(err),
            Error::DevicesNextDeviceError(err) => Some(err),
            Error::DevicesCreateError(_, err) => Some(err),
            Error::StreamingRemoteRenderConnectFail(_, err) => Some(err),
            Error::StreamingIdentifyLocalAddressError(err) => Some(err),
            Error::DLNASetAVTransportURIError(err) => Some(err),
            Error::DLNAPlayError(err) => Some(err),
            Error::DLNAStreamingError(err) => Some(err),
            _ => None,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
