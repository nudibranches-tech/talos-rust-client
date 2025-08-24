//! Support for loading Talos configuration files

use crate::error::{Error, Result};
use crate::TalosConnector;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, instrument};

/// Talos configuration file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TalosConfig {
    /// Current context name
    pub context: String,
    /// Available contexts
    pub contexts: HashMap<String, Context>,
}

/// Context configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    /// CA certificate (base64 encoded)
    pub ca: String,
    /// Client certificate (base64 encoded)
    pub crt: String,
    /// Client key (base64 encoded)
    pub key: String,
    /// Endpoints
    pub endpoints: Vec<String>,
    /// Nodes (optional)
    #[serde(default)]
    pub nodes: Vec<String>,
}

impl TalosConfig {
    /// Load a Talos configuration file
    #[instrument(skip(path), fields(path = ?path.as_ref()))]
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        debug!("Loading talosconfig");
        let content = std::fs::read_to_string(path)?;
        let config: TalosConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get the current context
    pub fn current_context(&self) -> Result<&Context> {
        self.contexts
            .get(&self.context)
            .ok_or_else(|| Error::TalosConfig(format!("Context '{}' not found", self.context)))
    }

    /// Get a specific context
    pub fn get_context(&self, name: &str) -> Result<&Context> {
        self.contexts
            .get(name)
            .ok_or_else(|| Error::TalosConfig(format!("Context '{name}' not found")))
    }

    /// Create a TalosConnector from the current context
    pub fn connector(&self) -> Result<TalosConnector> {
        self.context_connector(&self.context)
    }

    /// Create a TalosConnector from a specific context
    pub fn context_connector(&self, context_name: &str) -> Result<TalosConnector> {
        let context = self.get_context(context_name)?;
        context.connector()
    }
}

impl Context {
    /// Create a TalosConnector from this context
    pub fn connector(&self) -> Result<TalosConnector> {
        // Use the first endpoint
        let endpoint = self
            .endpoints
            .first()
            .ok_or_else(|| Error::TalosConfig("No endpoints configured".to_string()))?;

        // Decode certificates
        let ca = decode_base64(&self.ca)?;
        let cert = decode_base64(&self.crt)?;
        let key = decode_base64(&self.key)?;

        Ok(TalosConnector::new(endpoint)
            .ca_pem(ca)
            .cert_pem(cert)
            .key_pem(key))
    }

    /// Create TalosConnectors for all endpoints
    pub fn connectors(&self) -> Result<Vec<(String, TalosConnector)>> {
        let ca = decode_base64(&self.ca)?;
        let cert = decode_base64(&self.crt)?;
        let key = decode_base64(&self.key)?;

        Ok(self
            .endpoints
            .iter()
            .map(|endpoint| {
                let connector = TalosConnector::new(endpoint)
                    .ca_pem(ca.clone())
                    .cert_pem(cert.clone())
                    .key_pem(key.clone());
                (endpoint.clone(), connector)
            })
            .collect())
    }
}

fn decode_base64(data: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(Into::into)
}

/// Load a TalosConnector from the default talosconfig location
///
/// This looks for the config file at ~/.talos/config
pub fn from_default_config() -> Result<TalosConnector> {
    let home = std::env::var("HOME")
        .map_err(|_| Error::TalosConfig("HOME environment variable not set".to_string()))?;
    let config_path = Path::new(&home).join(".talos").join("config");

    if !config_path.exists() {
        return Err(Error::TalosConfig(format!(
            "Default config file not found at {config_path:?}"
        )));
    }

    let config = TalosConfig::from_file(config_path)?;
    config.connector()
}
