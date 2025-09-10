#!/bin/bash

# Coverage script for typefrog

set -e

COVERAGE_DIR="target/llvm-cov"
LCOV_FILE="lcov.info"

echo "🧪 Running tests with coverage..."

# Clean previous coverage data
cargo llvm-cov clean --workspace

# Run tests and generate coverage
echo "📊 Generating coverage report..."
cargo llvm-cov --all-features --workspace --html --lcov --output-path "$LCOV_FILE"

echo "✅ Coverage report generated!"
echo "📄 LCOV report: $LCOV_FILE"
echo "🌐 HTML report: $COVERAGE_DIR/html/index.html"

# Open HTML report if on macOS or Linux with display
if [[ "$OSTYPE" == "darwin"* ]]; then
    open "$COVERAGE_DIR/html/index.html"
elif [[ -n "${DISPLAY:-}" ]]; then
    xdg-open "$COVERAGE_DIR/html/index.html" 2>/dev/null || true
fi

echo ""
echo "Coverage summary:"
if command -v lcov >/dev/null 2>&1; then
    lcov --summary "$LCOV_FILE"
else
    echo "Install lcov for detailed coverage summary"
fi