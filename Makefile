# Makefile for typefrog development

.PHONY: help test coverage lint fmt check clean install-tools

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

test: ## Run all tests
	cargo test --all-features

coverage: ## Generate test coverage report (HTML)
	cargo llvm-cov --all-features --workspace --html
	@echo "Coverage report generated in target/llvm-cov/html/index.html"

coverage-lcov: ## Generate test coverage in lcov format
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	@echo "Coverage report generated in lcov.info"

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code with rustfmt
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

check: fmt-check lint test ## Run all checks (format, lint, test)

clean: ## Clean build artifacts
	cargo clean
	rm -f lcov.info

install-tools: ## Install required development tools
	cargo install cargo-llvm-cov
	rustup component add clippy rustfmt

# Development workflow
dev-setup: install-tools ## Set up development environment
	git submodule update --init --recursive

ci: check coverage-lcov ## Run CI pipeline locally