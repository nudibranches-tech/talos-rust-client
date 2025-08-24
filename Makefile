.PHONY: help test build release check fmt clippy clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

test: ## Run all tests
	cargo test --all-features

build: ## Build the project
	cargo build --all-features

check: ## Run cargo check
	cargo check --all-features

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy lints
	cargo clippy --all-features -- -D warnings

clean: ## Clean build artifacts
	cargo clean

release: ## Create a new release (usage: make release VERSION=0.1.1)
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION not specified"; \
		echo "Usage: make release VERSION=0.1.1"; \
		exit 1; \
	fi
	@./scripts/release.sh $(VERSION)
