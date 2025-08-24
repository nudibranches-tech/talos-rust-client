//! Error types for the Talos client

use thiserror::Error;

/// Result type alias for Talos client operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Talos client operations
#[derive(Error, Debug)]
pub enum Error {
    /// TLS configuration error
    #[error("TLS configuration error: {0}")]
    TlsConfig(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(#[from] tonic::transport::Error),

    /// Certificate parsing error
    #[error("Certificate parsing error: {0}")]
    CertParse(String),

    /// Key parsing error
    #[error("Key parsing error: {0}")]
    KeyParse(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Missing required configuration
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),

    /// Talosconfig parsing error
    #[cfg(feature = "talosconfig")]
    #[error("Talosconfig error: {0}")]
    TalosConfig(String),

    /// Base64 decode error
    #[cfg(feature = "talosconfig")]
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// YAML parsing error
    #[cfg(feature = "talosconfig")]
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Other errors
    #[error("{0}")]
    Other(String),
}
