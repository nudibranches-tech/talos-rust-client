//! Example: Stream Talos events

use futures_util::StreamExt;
use talos_rust_client::{MachineServiceClient, TalosConnector};
use tracing::{info, error};

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

    // Stream events
    info!("Streaming events (press Ctrl+C to stop)...");
    let request = tonic::Request::new(talos_rust_client::machine::EventsRequest {
        tail_events: 10,
        tail_id: String::new(),
        tail_seconds: 0,
        with_actor_id: String::new(),
    });

    let mut stream = client.events(request).await?.into_inner();

    let mut count = 0;
    while let Some(event) = stream.next().await {
        match event {
            Ok(event_msg) => {
                if let Some(metadata) = &event_msg.metadata {
                    info!("[{}] Node: {}", count, metadata.hostname);
                }
                
                info!("Event ID: {}", event_msg.id);
                if !event_msg.actor_id.is_empty() {
                    info!("Actor ID: {}", event_msg.actor_id);
                }
                
                if let Some(data) = &event_msg.data {
                    if let Ok(data_str) = String::from_utf8(data.value.clone()) {
                        info!("Data: {}", data_str);
                    }
                }

                count += 1;
                // Stop after 10 events for this example
                if count >= 10 {
                    info!("Received 10 events, stopping...");
                    break;
                }
            }
            Err(e) => {
                error!("Error receiving event: {}", e);
                break;
            }
        }
    }

    Ok(())
}