//! Example: Connect using a Talos config file

use talos_rust_client::{talosconfig::TalosConfig, MachineServiceClient};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load talosconfig from file
    let config_path = std::env::var("TALOSCONFIG").unwrap_or_else(|_| {
        let home = std::env::var("HOME").expect("HOME not set");
        format!("{home}/.talos/config")
    });

    info!("Loading talosconfig from: {}", config_path);
    let config = TalosConfig::from_file(config_path)?;

    info!("Current context: {}", config.context);
    let context = config.current_context()?;
    info!("Available endpoints: {:?}", context.endpoints);

    // Connect to Talos
    info!("Connecting to Talos...");
    let channel = config.connector()?.connect().await?;

    // Create machine service client
    let mut client = MachineServiceClient::new(channel);

    // Get version
    info!("Getting version information...");
    let request = tonic::Request::new(talos_rust_client::generated::google::protobuf::Empty {});
    let response = client.version(request).await?;

    let version = response.into_inner();
    info!("Connected successfully!");

    for msg in &version.messages {
        if let Some(metadata) = &msg.metadata {
            info!("Node: {}", metadata.hostname);
        }
        if let Some(version_info) = &msg.version {
            info!("  Talos: {}", version_info.tag);
        }
    }

    Ok(())
}
