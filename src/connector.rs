//! Connection builder for Talos gRPC API with mTLS support

use crate::error::{Error, Result};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};
use tracing::{debug, instrument};

/// Connection builder for Talos gRPC API
#[derive(Debug, Clone)]
pub struct TalosConnector {
    endpoint: String,
    ca_cert: Option<Vec<u8>>,
    client_cert: Option<Vec<u8>>,
    client_key: Option<Vec<u8>>,
    server_name: Option<String>,
}

impl TalosConnector {
    /// Create a new connection builder
    ///
    /// # Arguments
    /// * `endpoint` - The Talos API endpoint (e.g., "https://192.168.1.100:50000")
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            ca_cert: None,
            client_cert: None,
            client_key: None,
            server_name: None,
        }
    }

    /// Set the CA certificate in PEM format
    pub fn ca_pem(mut self, pem: Vec<u8>) -> Self {
        self.ca_cert = Some(pem);
        self
    }

    /// Set the CA certificate from a file path
    pub fn ca_pem_file(mut self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let pem = std::fs::read(path)?;
        self.ca_cert = Some(pem);
        Ok(self)
    }

    /// Set the client certificate in PEM format
    pub fn cert_pem(mut self, pem: Vec<u8>) -> Self {
        self.client_cert = Some(pem);
        self
    }

    /// Set the client certificate from a file path
    pub fn cert_pem_file(mut self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let pem = std::fs::read(path)?;
        self.client_cert = Some(pem);
        Ok(self)
    }

    /// Set the client private key in PEM format
    pub fn key_pem(mut self, pem: Vec<u8>) -> Self {
        self.client_key = Some(pem);
        self
    }

    /// Set the client private key from a file path
    pub fn key_pem_file(mut self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let pem = std::fs::read(path)?;
        self.client_key = Some(pem);
        Ok(self)
    }

    /// Set the server name for SNI (Server Name Indication)
    pub fn server_name(mut self, name: impl Into<String>) -> Self {
        self.server_name = Some(name.into());
        self
    }

    /// Connect to the Talos API
    #[instrument(skip(self))]
    pub async fn connect(self) -> Result<Channel> {
        debug!("Connecting to Talos API at {}", self.endpoint);

        // Validate required fields
        let ca_cert = self
            .ca_cert
            .ok_or_else(|| Error::MissingConfig("CA certificate".to_string()))?;
        let client_cert = self
            .client_cert
            .ok_or_else(|| Error::MissingConfig("Client certificate".to_string()))?;
        let client_key = self
            .client_key
            .ok_or_else(|| Error::MissingConfig("Client key".to_string()))?;

        // Create tonic Certificate and Identity
        let ca = Certificate::from_pem(ca_cert);
        let identity = Identity::from_pem(client_cert, client_key);

        // Configure TLS
        let mut tls_config = ClientTlsConfig::new().ca_certificate(ca).identity(identity);

        // Set domain name if provided, otherwise extract from endpoint
        if let Some(domain) = self.server_name {
            tls_config = tls_config.domain_name(domain);
        } else {
            // Extract domain from endpoint URL
            if let Ok(url) = url::Url::parse(&self.endpoint) {
                if let Some(host) = url.host_str() {
                    tls_config = tls_config.domain_name(host);
                }
            }
        }

        let channel = Endpoint::from_shared(self.endpoint.clone())
            .map_err(|e| Error::Other(format!("Invalid endpoint: {e}")))?
            .tls_config(tls_config)
            .map_err(|e| Error::TlsConfig(format!("Failed to set TLS config: {e}")))?
            .connect()
            .await?;

        debug!("Successfully connected to Talos API");
        Ok(channel)
    }
}
