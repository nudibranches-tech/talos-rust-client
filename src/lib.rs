//! Rust gRPC client for SideroLabs Talos
//!
//! This crate provides a Rust client for the Talos gRPC API with mTLS support.
//!
//! # Example
//!
//! ```no_run
//! use talos_rust_client::TalosConnector;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ca = std::fs::read("ca.crt")?;
//!     let cert = std::fs::read("client.crt")?;
//!     let key = std::fs::read("client.key")?;
//!
//!     let channel = TalosConnector::new("https://192.168.1.100:50000")
//!         .ca_pem(ca)
//!         .cert_pem(cert)
//!         .key_pem(key)
//!         .connect()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

pub mod connector;
pub mod error;

#[cfg(feature = "talosconfig")]
pub mod talosconfig;

// Include generated proto modules
#[allow(missing_docs)]
#[allow(clippy::all)]
#[allow(rustdoc::invalid_rust_codeblocks)]
#[allow(rustdoc::broken_intra_doc_links)]
#[allow(rustdoc::bare_urls)]
#[doc(hidden)]
pub mod generated {
    include!("generated/mod.rs");
}

// Re-export proto modules at crate root
pub use generated::securityapi as security;
pub use generated::{cluster, common, inspect, machine, storage, time};

pub use connector::TalosConnector;
pub use error::{Error, Result};

// Re-export commonly used client types
pub use cluster::cluster_service_client::ClusterServiceClient;
pub use inspect::inspect_service_client::InspectServiceClient;
pub use machine::machine_service_client::MachineServiceClient;
pub use security::security_service_client::SecurityServiceClient;
pub use storage::storage_service_client::StorageServiceClient;
pub use time::time_service_client::TimeServiceClient;

// Re-export tonic types for convenience
pub use tonic;
pub use tonic::transport::Channel;
