use std::fmt;

#[derive(Debug)]
pub enum Error {
    DevicesDiscoverFail(rupnp::Error),
    DevicesNextDeviceError(rupnp::Error),
    DevicesUrlParseError(String),
    DevicesCreateError(String, rupnp::Error),
    StreamingHostParseError(String),
    StreamingFileDoesNotExist(String),
    StreamingRemoteRenderConnectFail(String, std::io::Error),
    StreamingIdentifyLocalAddressError(std::io::Error),
    DLNASetAVTransportURIError(rupnp::Error),
    DLNAPlayError(rupnp::Error),
    DLNAStreamingError(tokio::task::JoinError),
    CLIDeviceNotFound(String),
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
                url, 
                err
            ),
            Error::StreamingHostParseError(host) => write!(f, "Failed to parse host '{}'", host),
            Error::StreamingFileDoesNotExist(file) => write!(f, "File '{}' does not exist", file),
            Error::StreamingRemoteRenderConnectFail(host, err) => write!(
                f,
                "Failed to connect to remote render '{}': {}",
                host,
                err
            ),
            Error::StreamingIdentifyLocalAddressError(err) => write!(
                f,
                "Failed to identify local address: {}",
                err
            ),
            Error::DLNASetAVTransportURIError(err) => write!(f, "Failed to set AVTransportURI: {}", err),
            Error::DLNAPlayError(err) => write!(f, "Failed to Play: {}", err),
            Error::DLNAStreamingError(err) => write!(f, "Failed to stream: {}", err),
            Error::CLIDeviceNotFound(message) => write!(f, "Device not found ({})", message),
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
