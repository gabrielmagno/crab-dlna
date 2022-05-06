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

    pub async fn find_all(duration_secs: u64) -> Vec<Self> {

        let search_target = SearchTarget::URN(AV_TRANSPORT);
        let devices = match rupnp::discover(&search_target, Duration::from_secs(duration_secs)).await {
            Ok(devices) => devices,
            Err(e) => panic!("Failed to discover devices, error: {}", e)
        };
        
        pin_utils::pin_mut!(devices);
    
        let mut renders = Vec::new();
    
        while let Some(device) = devices.try_next().await.unwrap_or_else(|e| panic!("Failed to get next device, error: {}", e)) {
            match Self::from_device(device).await {
                Some(render) => { renders.push(render); }
                None => {}
            };
        }
    
        return renders;
    }

    pub async fn select_by_query(duration_secs: u64, query: &str) -> Option<Self> {
        for render in Self::find_all(duration_secs).await {
            let render_str = render.to_string();
            if render_str.contains(query) {
                return Some(render);
            }
        }
        None
    }

    pub async fn select_by_url(url: &'static str) -> Option<Self> {

        let uri = Uri::from_static(url);
        
        let device = rupnp::Device::from_url(uri)
            .await
            .unwrap_or_else(
                |e|
                panic!("Failed to retrieve device from {}, error: {}", url, e)
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
