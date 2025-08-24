# Claude Project Context

This file provides context for Claude AI when working on the talos-rust-client project.

## Project Overview

`talos-rust-client` is a Rust gRPC client library for SideroLabs Talos with mTLS support. It provides type-safe bindings to the Talos API, enabling secure communication with Talos nodes.

## Key Features

- mTLS (mutual TLS) authentication required - no plaintext connections
- Pre-generated protobuf bindings from Talos v1.10.6
- Async/await support with tokio
- Optional talosconfig file support
- Pure Rust implementation with rustls

## Project Structure

```
.
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── connector.rs        # TalosConnector for mTLS connections
│   ├── error.rs           # Error types
│   ├── talosconfig.rs     # Talosconfig file support (optional feature)
│   └── generated/         # Pre-generated protobuf code
│       ├── mod.rs         # Module definitions
│       └── *.rs           # Generated service/message definitions
├── examples/              # Example usage
│   ├── version.rs         # Get Talos version
│   ├── health.rs          # Check cluster health
│   ├── events.rs          # Stream events
│   └── talosconfig_connect.rs # Connect using talosconfig
├── vendor/talos/          # Talos submodule (v1.10.6)
└── .github/workflows/     # GitHub Actions CI/CD

```

## Important Commands

```bash
# Run tests
cargo test --all-features

# Build
cargo build --all-features

# Run lints
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt

# Create a release (from tag)
./scripts/release.sh 0.1.0
# or
make release VERSION=0.1.0

# Run examples
cargo run --example version
cargo run --example health --features talosconfig
```

## Design Decisions

1. **Pre-generated Protos**: We use pre-generated protobuf code in `src/generated/` instead of building at compile time to avoid protoc dependency.

2. **No Plaintext**: The connector only supports mTLS connections for security.

3. **Rustls Only**: We use rustls instead of OpenSSL for pure Rust implementation.

4. **Doc Tests Disabled**: Due to non-Rust code examples in generated protobuf documentation, we disable doc tests in Cargo.toml.

## Release Process

Releases are triggered by git tags:

1. Push a tag like `v0.1.0`
2. GitHub Actions will:
   - Update Cargo.toml version using cargo-edit
   - Run tests
   - Create GitHub release
   - Publish to crates.io
   - Build Linux binaries (amd64, musl, arm64)

## Known Issues

- Doc tests are disabled due to generated protobuf files containing non-Rust code examples
- The generated code requires serde as a non-optional dependency

## Testing

For integration testing, mock certificates can be generated:

```bash
# Generate test CA
openssl req -x509 -newkey rsa:4096 -keyout ca-key.pem -out ca.crt -days 365 -nodes -subj "/CN=Test CA"

# Generate client cert
openssl req -newkey rsa:4096 -keyout client-key.pem -out client.csr -nodes -subj "/CN=Test Client"
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca-key.pem -CAcreateserial -out client.crt -days 365
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to this project.
