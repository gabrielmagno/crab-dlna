use futures::prelude::*;
use rupnp::ssdp::{SearchTarget, URN};
use std::time::Duration;
use http::Uri;
use crate::{Error, Result};

const AV_TRANSPORT: URN = URN::service("schemas-upnp-org", "AVTransport", 1);

#[derive(Debug, Clone)]
pub struct Render {
    pub device: rupnp::Device,
    pub service: rupnp::Service,
}

impl Render {

    async fn from_device(device: rupnp::Device) -> Option<Self> {
        match device.find_service(&AV_TRANSPORT) {
            Some(service) => Some(
                Self{
                    device: device.clone(),
                    service: service.clone(),
                }
            ),
            None => {
                println!("WARNING: No AVTransport service found on {}", device.friendly_name());
                None
            }
        }
    }

    pub async fn find_all(duration_secs: u64) -> Result<Vec<Self>> {

        let search_target = SearchTarget::URN(AV_TRANSPORT);
        let devices = rupnp::discover(&search_target, Duration::from_secs(duration_secs))
            .await
            .map_err(|err| Error::DevicesDiscoverFail(err))?;

        pin_utils::pin_mut!(devices);
    
        let mut renders = Vec::new();
    
        while let Some(device) = devices.try_next().await.map_err(|err| Error::DevicesNextDeviceError(err))? { 
            match Self::from_device(device).await {
                Some(render) => { renders.push(render); }
                None => {}
            };
        }
    
        return Ok(renders);
    }

    pub async fn select_by_query(duration_secs: u64, query: &String) -> Result<Option<Self>> {
        for render in Self::find_all(duration_secs).await? {
            let render_str = render.to_string();
            if render_str.contains(query.as_str()) {
                return Ok(Some(render));
            }
        }
        Ok(None)
    }

    pub async fn select_by_url(url: &String) -> Result<Option<Self>> {

        let uri: Uri = url.parse()
            .map_err(|_| Error::DevicesUrlParseError(url.to_owned()))?;
        
        let device = rupnp::Device::from_url(uri)
            .await
            .map_err(|err| Error::DevicesCreateError(url.to_owned(), err))?;
    
        Ok(Self::from_device(device).await)
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
