# Contributing to talos-rust-client

Thank you for your interest in contributing to talos-rust-client! This document provides guidelines and instructions for contributing.

## Development Setup

1. Clone the repository with submodules:
   ```bash
   git clone --recursive https://github.com/nuditech/talos-rust-client.git
   cd talos-rust-client
   ```

2. Ensure you have Rust installed (MSRV 1.75+):
   ```bash
   rustup update stable
   ```

3. Build the project:
   ```bash
   cargo build
   ```

## Updating the Talos Submodule

The Talos API definitions are included as a git submodule. To update to a new version:

1. Navigate to the submodule directory:
   ```bash
   cd vendor/talos
   ```

2. Fetch the latest tags:
   ```bash
   git fetch --tags
   ```

3. Checkout the desired tag:
   ```bash
   git checkout v1.11.0  # Replace with desired version
   ```

4. Return to the main directory and commit the change:
   ```bash
   cd ../..
   git add vendor/talos
   git commit -m "chore: update Talos submodule to v1.11.0"
   ```

5. Update the build.rs file if new proto files were added or removed.

6. Regenerate the proto bindings:
   ```bash
   cargo build
   ```

7. Update the README.md to reflect the new pinned version.

## Code Style

- Format code with `cargo fmt`
- Run `cargo clippy -- -D warnings` before submitting
- Add documentation comments for public APIs
- Include examples in doc comments where appropriate

## Testing

- Write unit tests for new functionality
- Integration tests require a running Talos cluster
- Set environment variables for integration tests:
  ```bash
  export TALOS_ENDPOINT="https://192.168.1.100:50000"
  export TALOS_CA="path/to/ca.crt"
  export TALOS_CERT="path/to/client.crt"
  export TALOS_KEY="path/to/client.key"
  ```

## Pull Request Process

1. Fork the repository and create a feature branch
2. Make your changes following the code style guidelines
3. Add or update tests as needed
4. Update documentation if APIs change
5. Run the test suite and ensure all checks pass
6. Submit a pull request with a clear description

## Commit Message Format

Follow conventional commits:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `chore:` for maintenance tasks
- `test:` for test additions/changes

## Security

- Never commit certificates, keys, or sensitive data
- Report security issues privately to the maintainers
- Ensure examples use placeholder values for sensitive data

## Questions?

Feel free to open an issue for questions or discussions about potential contributions.
