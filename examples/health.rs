//! Example: Check cluster health

use talos_rust_client::{ClusterServiceClient, TalosConnector};
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

    // Create cluster service client
    let mut client = ClusterServiceClient::new(channel);

    // Check health
    info!("Checking cluster health...");
    let request = tonic::Request::new(talos_rust_client::cluster::HealthCheckRequest {
        wait_timeout: Some(talos_rust_client::generated::google::protobuf::Duration {
            seconds: 30,
            nanos: 0,
        }),
        cluster_info: None,
    });

    let response = client.health_check(request).await?;
    let mut stream = response.into_inner();

    info!("Cluster Health Progress:");
    while let Some(progress) = stream
        .message()
        .await?
    {
        if let Some(metadata) = &progress.metadata {
            info!("Node: {}", metadata.hostname);
        }
        info!("Progress: {}", progress.message);
        
        // Check if the message indicates cluster is ready
        if progress.message.contains("ready") || progress.message.contains("healthy") {
            info!("Cluster is ready!");
            break;
        }
    }

    Ok(())
}