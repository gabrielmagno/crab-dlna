use futures::prelude::*;
use rupnp::ssdp::{SearchTarget, URN};
use std::time::Duration;
use http::Uri;

const AV_TRANSPORT: URN = URN::service("schemas-upnp-org", "AVTransport", 1);

#[derive(Debug, Clone)]
pub struct Render {
    pub device: rupnp::Device,
    pub service: rupnp::Service,
}

impl Render {

    pub async fn from_device(device: rupnp::Device) -> Option<Self> {
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

    pub async fn find_all(duration_secs: u64) -> Result<Vec<Self>, rupnp::Error> {

        let search_target = SearchTarget::URN(AV_TRANSPORT);
        let devices = rupnp::discover(&search_target, Duration::from_secs(duration_secs)).await?;
        pin_utils::pin_mut!(devices);
    
        let mut renders = Vec::new();
    
        while let Some(device) = devices.try_next().await? {
            match Self::from_device(device).await {
                Some(render) => { renders.push(render); }
                None => {}
            }
        }
    
        return Ok(renders);
    }

    pub async fn select_by_query(duration_secs: u64, query: &str) -> Result<Option<Self>, rupnp::Error> {
        let renders = Self::find_all(duration_secs).await?;
        for render in renders {
            let render_str = render.to_string();
            if render_str.contains(query) {
                return Ok(Some(render));
            }
        }
        Ok(None)
    }

    pub async fn select_by_url(url: &'static str) -> Option<Self> {

        let uri = Uri::from_static(url);
        
        let device = rupnp::Device::from_url(uri)
            .await
            .expect(
                format!("Failed to retrieve device from {}", url).as_str()
            );
    
        Self::from_device(device).await
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
