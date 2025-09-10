#!/bin/bash

# Validation script to test coverage and clippy setup

set -e

echo "🔍 Validating typefrog development setup..."

# Check required tools
echo "📋 Checking required tools..."

check_tool() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "  ✅ $1 is available"
    else
        echo "  ❌ $1 is missing"
        return 1
    fi
}

check_tool rustc
check_tool cargo
check_tool git

# Check Rust components
echo "📦 Checking Rust components..."
if rustup component list --installed | grep -q clippy; then
    echo "  ✅ clippy is installed"
else
    echo "  ❌ clippy is not installed"
    echo "    Run: rustup component add clippy"
    exit 1
fi

if rustup component list --installed | grep -q rustfmt; then
    echo "  ✅ rustfmt is installed" 
else
    echo "  ❌ rustfmt is not installed"
    echo "    Run: rustup component add rustfmt"
    exit 1
fi

# Check cargo-llvm-cov
if cargo llvm-cov --version >/dev/null 2>&1; then
    echo "  ✅ cargo-llvm-cov is installed"
else
    echo "  ❌ cargo-llvm-cov is not installed"
    echo "    Run: cargo install cargo-llvm-cov"
    exit 1
fi

# Check configuration files
echo "📄 Checking configuration files..."

check_file() {
    if [[ -f "$1" ]]; then
        echo "  ✅ $1 exists"
    else
        echo "  ❌ $1 is missing"
        return 1
    fi
}

check_file "Cargo.toml"
check_file "clippy.toml"
check_file "rustfmt.toml"
check_file ".github/workflows/ci.yml"
check_file "Makefile"

# Check git submodules
echo "🔗 Checking git submodules..."
if git submodule status | grep -q "^-"; then
    echo "  ⚠️  Git submodules are not initialized"
    echo "    Run: git submodule update --init --recursive"
    echo "    Note: This may be required for the project to build"
else
    echo "  ✅ Git submodules appear to be initialized"
fi

# Test clippy configuration (dry-run)
echo "🧹 Testing clippy configuration..."
if cargo clippy --version >/dev/null 2>&1; then
    echo "  ✅ Clippy can be invoked"
else
    echo "  ❌ Clippy cannot be invoked"
    exit 1
fi

# Test rustfmt configuration
echo "🎨 Testing rustfmt configuration..."
if cargo fmt --version >/dev/null 2>&1; then
    echo "  ✅ Rustfmt can be invoked"
else
    echo "  ❌ Rustfmt cannot be invoked"
    exit 1
fi

echo ""
echo "✅ Development setup validation complete!"
echo ""
echo "Next steps:"
echo "  1. Initialize git submodules if needed:"
echo "     git submodule update --init --recursive"
echo "  2. Try building the project:"
echo "     cargo build"
echo "  3. Run tests:"
echo "     make test"
echo "  4. Generate coverage:"
echo "     make coverage"
echo "  5. Run linting:"
echo "     make lint"
echo ""
echo "See README.md for detailed instructions."