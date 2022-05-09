use futures::prelude::*;
use log::{info, warn,debug};
use std::time::Duration;
use http::Uri;
use rupnp::ssdp::{SearchTarget, URN};
use crate::error::{Error, Result};

const AV_TRANSPORT: URN = URN::service("schemas-upnp-org", "AVTransport", 1);

macro_rules! format_device{
    ($device:expr)=>{
        {
            format!(
                "[{}] {} @ {}", 
                $device.device_type(), 
                $device.friendly_name(), 
                $device.url()
            )
        }
    }
}

/// A DLNA device which is capable of AVTransport actions.
#[derive(Debug, Clone)]
pub struct Render {
    /// The UPnP device
    pub device: rupnp::Device,
    /// The AVTransport service
    pub service: rupnp::Service,
}

/// An specification of a DLNA render device.
#[derive(Debug, Clone)]
pub enum RenderSpec {
    /// Render specified by a location URL
    Location(String),
    /// Render specified by a query string
    Query(u64, String),
    /// The first render found
    First(u64),
}

impl Render {

    /// Create a new render from render device specification.
    pub async fn new(render_spec: RenderSpec) -> Result<Self> {
        match &render_spec {
            RenderSpec::Location(device_url) => {
                info!("Render specified by location: {}", device_url);
                Self::select_by_url(&device_url).await?
                    .ok_or(Error::DevicesRenderNotFound(render_spec))
            },
            RenderSpec::Query(timeout, device_query) => {
                info!("Render specified by query: {}", device_query);
                Self::select_by_query(*timeout, &device_query).await?
                    .ok_or(Error::DevicesRenderNotFound(render_spec))
            },
            RenderSpec::First(timeout) => {
                info!("No render specified, selecting first one");
                Ok(
                    Self::discover(*timeout)
                        .await?
                        .first()
                        .ok_or(Error::DevicesRenderNotFound(render_spec))?
                        .to_owned()
                )
            }
        }
    }

    /// Discovers DLNA device with AVTransport on the network.
    pub async fn discover(duration_secs: u64) -> Result<Vec<Self>> {

        info!("Discovering devices in the network, waiting {} seconds...", duration_secs);
        let search_target = SearchTarget::URN(AV_TRANSPORT);
        let devices = rupnp::discover(&search_target, Duration::from_secs(duration_secs))
            .await
            .map_err(|err| Error::DevicesDiscoverFail(err))?;

        pin_utils::pin_mut!(devices);
    
        let mut renders = Vec::new();
    

        while let Some(device) = devices.try_next().await.map_err(|err| Error::DevicesNextDeviceError(err))? {
            debug!("Found device: {}", format_device!(device)); 
            match Self::from_device(device).await {
                Some(render) => { renders.push(render); }
                None => {}
            };
        }
    
        return Ok(renders);
    }

    /// Returns the host of the render
    pub fn host(&self) -> String {
        self.device.url().authority().unwrap().host().to_string()
    }

    async fn select_by_url(url: &String) -> Result<Option<Self>> {
        debug!("Selecting device by url: {}", url);
        let uri: Uri = url.parse()
            .map_err(|_| Error::DevicesUrlParseError(url.to_owned()))?;
        
        let device = rupnp::Device::from_url(uri)
            .await
            .map_err(|err| Error::DevicesCreateError(url.to_owned(), err))?;
    
        Ok(Self::from_device(device).await)
    }

    async fn select_by_query(duration_secs: u64, query: &String) -> Result<Option<Self>> {
        debug!("Selecting device by query: '{}'", query);
        for render in Self::discover(duration_secs).await? {
            let render_str = render.to_string();
            if render_str.contains(query.as_str()) {
                return Ok(Some(render));
            }
        }
        Ok(None)
    }

    async fn from_device(device: rupnp::Device) -> Option<Self> {
        debug!("Retrieving AVTransport service from device '{}'", format_device!(device));
        match device.find_service(&AV_TRANSPORT) {
            Some(service) => Some(
                Self{
                    device: device.clone(),
                    service: service.clone(),
                }
            ),
            None => {
                warn!("No AVTransport service found on {}", device.friendly_name());
                None
            }
        }
    }
}

impl std::fmt::Display for Render {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}][{}] {} @ {}",
            self.device.device_type(),
            self.service.service_type(),
            self.device.friendly_name(),
            self.device.url()
        )
    }
}
