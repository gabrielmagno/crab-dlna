use crate::{
    devices::Render,
    error::{Error, Result},
    streaming::MediaStreamingServer,
};
use log::{debug, info};
use xml::escape::escape_str_attribute;
use tokio::time::{sleep, Duration};

const PAYLOAD_PLAY: &str = r#"
    <InstanceID>0</InstanceID>
    <Speed>1</Speed>
"#;

const STREAMING_SERVER_WAIT_SECS: u64 = 5;

/// Plays a media file in a DLNA compatible device render, according to the render and media streaming server provided
pub async fn play(render: Render, streaming_server: MediaStreamingServer) -> Result<()> {
    let subtitle_uri = streaming_server.subtitle_uri();
    let payload_subtitle = match subtitle_uri {
        Some(subtitle_uri) => {
            escape_str_attribute(
                format!(r###"
                    <DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/"
                               xmlns:dc="http://purl.org/dc/elements/1.1/" 
                               xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/" 
                               xmlns:dlna="urn:schemas-dlna-org:metadata-1-0/" 
                               xmlns:sec="http://www.sec.co.kr/" 
                               xmlns:xbmc="urn:schemas-xbmc-org:metadata-1-0/">
                        <item id="0" parentID="-1" restricted="1">
                            <dc:title>nano-dlna Video</dc:title>
                            <res protocolInfo="http-get:*:video/{type_video}:" xmlns:pv="http://www.pv.com/pvns/" pv:subtitleFileUri="{uri_sub}" pv:subtitleFileType="{type_sub}">{uri_video}</res>
                            <res protocolInfo="http-get:*:text/srt:*">{uri_sub}</res>
                            <res protocolInfo="http-get:*:smi/caption:*">{uri_sub}</res>
                            <sec:CaptionInfoEx sec:type="{type_sub}">{uri_sub}</sec:CaptionInfoEx>
                            <sec:CaptionInfo sec:type="{type_sub}">{uri_sub}</sec:CaptionInfo>
                            <upnp:class>object.item.videoItem.movie</upnp:class>
                        </item>
                    </DIDL-Lite>
                    "###,
                    uri_video = streaming_server.video_uri(),
                    type_video = streaming_server.video_type(),
                    uri_sub = subtitle_uri,
                    type_sub = streaming_server.subtitle_type().unwrap_or_else(|| "unknown".to_string())
                ).as_str()
           ).to_string()
        }
        None => "".to_string()
    };
    debug!("Subtitle payload: '{}'", payload_subtitle);

    let payload_setavtransporturi = format!(
        r#"
        <InstanceID>0</InstanceID>
        <CurrentURI>{}</CurrentURI>
        <CurrentURIMetaData>{}</CurrentURIMetaData>
        "#,
        streaming_server.video_uri(),
        payload_subtitle
    );
    debug!("SetAVTransportURI payload: '{}'", payload_setavtransporturi);

    info!("Starting media streaming server...");
    let streaming_server_handle = tokio::spawn(async move { streaming_server.run().await });

    debug!("Explicitly waiting {} seconds for streaming server to finish loading", STREAMING_SERVER_WAIT_SECS);
    sleep(Duration::from_secs(STREAMING_SERVER_WAIT_SECS)).await;

    info!("Setting Video URI");
    render
        .service
        .action(
            render.device.url(),
            "SetAVTransportURI",
            payload_setavtransporturi.as_str(),
        )
        .await
        .map_err(Error::DLNASetAVTransportURIError)?;

    info!("Playing video");
    render
        .service
        .action(render.device.url(), "Play", PAYLOAD_PLAY)
        .await
        .map_err(Error::DLNAPlayError)?;

    streaming_server_handle
        .await
        .map_err(Error::DLNAStreamingError)?;

    Ok(())
}
