# talos-rust-client

[![Crates.io](https://img.shields.io/crates/v/talos-rust-client.svg)](https://crates.io/crates/talos-rust-client)
[![Documentation](https://docs.rs/talos-rust-client/badge.svg)](https://docs.rs/talos-rust-client)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/talos-rust-client.svg)](#license)

Rust gRPC client for [SideroLabs Talos](https://github.com/siderolabs/talos) with mTLS support.

## Features

- **mTLS by default** - Secure communication with Talos nodes
- **Type-safe API** - Generated from official Talos proto files
- **Async/await** - Built on tokio and tonic
- **talosconfig support** - Load credentials from Talos config files
- **Rustls** - Pure Rust TLS implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
talos-rust-client = "0.1"
```

To use talosconfig file support:

```toml
[dependencies]
talos-rust-client = { version = "0.1", features = ["talosconfig"] }
```

## Quick Start

### Using certificates directly

```rust
use talos_rust_client::{TalosConnector, machine::MachineServiceClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read certificates
    let ca = std::fs::read("ca.crt")?;
    let cert = std::fs::read("client.crt")?;
    let key = std::fs::read("client.key")?;

    // Connect to Talos
    let channel = TalosConnector::new("https://192.168.1.100:50000")
        .ca_pem(ca)
        .cert_pem(cert)
        .key_pem(key)
        .connect()
        .await?;

    // Create a client
    let mut client = MachineServiceClient::new(channel);

    // Get version
    let request = tonic::Request::new(talos_rust_client::common::EmptyRequest {});
    let response = client.version(request).await?;
    
    Ok(())
}
```

### Using talosconfig

```rust
use talos_rust_client::talosconfig::TalosConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from default location (~/.talos/config)
    let config = TalosConfig::from_file("~/.talos/config")?;
    
    // Connect using current context
    let channel = config.connector()?.connect().await?;
    
    // Use the channel with any service client...
    
    Ok(())
}
```

## Examples

See the [examples](examples/) directory for more detailed examples:

- [`version.rs`](examples/version.rs) - Get Talos version information
- [`health.rs`](examples/health.rs) - Check cluster health
- [`events.rs`](examples/events.rs) - Stream Talos events
- [`talosconfig_connect.rs`](examples/talosconfig_connect.rs) - Connect using talosconfig

Run examples with:

```bash
# Using environment variables for certs
export TALOS_ENDPOINT="https://192.168.1.100:50000"
export TALOS_CA="ca.crt"
export TALOS_CERT="client.crt"
export TALOS_KEY="client.key"
cargo run --example version

# Using talosconfig
cargo run --example talosconfig_connect --features talosconfig
```

## Version Pinning

This crate includes Talos API definitions as a git submodule pinned to a specific release tag.
The current version is pinned to **v1.10.6**.

To update the submodule to a different tag, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

This crate enforces mTLS (mutual TLS) for all connections. There is no option to disable TLS or certificate verification. Always ensure your certificates and keys are kept secure and never logged or exposed.

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.