//! Example: Get Talos version information

use talos_rust_client::{MachineServiceClient, TalosConnector};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Read certificates from environment or files
    let endpoint = std::env::var("TALOS_ENDPOINT")
        .unwrap_or_else(|_| "https://192.168.1.100:50000".to_string());

    let ca = std::fs::read(std::env::var("TALOS_CA").unwrap_or_else(|_| "ca.crt".to_string()))?;
    let cert =
        std::fs::read(std::env::var("TALOS_CERT").unwrap_or_else(|_| "client.crt".to_string()))?;
    let key =
        std::fs::read(std::env::var("TALOS_KEY").unwrap_or_else(|_| "client.key".to_string()))?;

    // Connect to Talos
    info!("Connecting to Talos at {}", endpoint);
    let channel = TalosConnector::new(endpoint)
        .ca_pem(ca)
        .cert_pem(cert)
        .key_pem(key)
        .connect()
        .await?;

    // Create machine service client
    let mut client = MachineServiceClient::new(channel);

    // Get version
    info!("Getting version information...");
    let request = tonic::Request::new(talos_rust_client::generated::google::protobuf::Empty {});
    let response = client.version(request).await?;

    let version = response.into_inner();
    info!("Talos Version Information:");

    for msg in &version.messages {
        if let Some(metadata) = &msg.metadata {
            info!("Node: {}", metadata.hostname);
        }
        if let Some(version_info) = &msg.version {
            info!("  Tag: {}", version_info.tag);
            info!("  SHA: {}", version_info.sha);
            info!("  Built: {}", version_info.built);
            info!("  Go Version: {}", version_info.go_version);
            info!("  OS: {}", version_info.os);
            info!("  Arch: {}", version_info.arch);
        }
    }

    Ok(())
}
