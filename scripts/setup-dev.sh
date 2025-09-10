#!/bin/bash

# Development setup script for typefrog

set -e

echo "Setting up typefrog development environment..."

# Install required Rust components
echo "Installing Rust components..."
rustup component add clippy rustfmt

# Install cargo tools
echo "Installing cargo tools..."
cargo install cargo-llvm-cov

# Initialize git submodules (with timeout)
echo "Initializing git submodules..."
if timeout 120s git submodule update --init --recursive; then
    echo "✅ Git submodules initialized successfully"
else
    echo "⚠️  Git submodule initialization timed out or failed"
    echo "   You may need to initialize submodules manually:"
    echo "   git submodule update --init --recursive"
fi

# Try to build the project
echo "Testing project build..."
if cargo check; then
    echo "✅ Project builds successfully"
else
    echo "⚠️  Project build failed - check dependencies"
fi

echo "✅ Development environment setup complete!"
echo ""
echo "Available commands:"
echo "  make help     - Show all available make targets"
echo "  make test     - Run tests"
echo "  make coverage - Generate coverage report"  
echo "  make lint     - Run clippy"
echo "  make fmt      - Format code"