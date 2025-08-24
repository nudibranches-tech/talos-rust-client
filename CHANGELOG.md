# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-22

### Added

- Initial release of talos-rust-client
- mTLS connection support using rustls
- Generated client code from Talos v1.10.6 proto files
- Connection builder with flexible configuration options
- Support for loading credentials from talosconfig files (optional feature)
- Examples for common use cases:
  - Getting version information
  - Checking cluster health
  - Streaming events
  - Connecting via talosconfig
- Comprehensive error handling with custom error types
- Full async/await support with tokio runtime
- Type-safe API with serde serialization support
- CI/CD pipeline with GitHub Actions
- Dual licensing under MIT OR Apache-2.0

[0.1.0]: https://github.com/nuditech/talos-rust-client/releases/tag/v0.1.0