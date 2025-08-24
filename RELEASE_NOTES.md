# Release v0.1.0

Initial release of `talos-rust-client` - a Rust gRPC client for SideroLabs Talos with mTLS support.

## Features

- **mTLS Support**: Secure communication with Talos nodes using mutual TLS authentication
- **Type-safe API**: Generated from official Talos v1.10.6 proto files  
- **Async/await**: Built on tokio and tonic for modern async Rust
- **Talosconfig Support**: Optional feature to load credentials from Talos config files
- **Pure Rust TLS**: Uses rustls for TLS implementation
- **Pre-compiled Protos**: No protoc required at build time

## What's Included

- Connection builder with flexible mTLS configuration
- Support for all major Talos services:
  - Machine service (version, events, etc.)
  - Cluster service (health checks)
  - Inspect service
  - Security service
  - Storage service
  - Time service
- Comprehensive examples for common use cases
- Full API documentation
- CI/CD with GitHub Actions

## Installation

```toml
[dependencies]
talos-rust-client = "0.1.0"

# With talosconfig support
talos-rust-client = { version = "0.1.0", features = ["talosconfig"] }
```

## Next Steps

Future releases will include:
- Additional service coverage
- Helper utilities for common operations
- Integration tests
- Performance optimizations

For more information, see the [README](README.md) and [examples](examples/).