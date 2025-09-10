# typefrog

[Typefrog](https://fistfulofbytes.com/typefrog/) is a ~datafrog~ [`ascent`](https://s-arash.github.io/ascent/) library for analyzing typeql schemas.

> [!CAUTION]
> Here be dragons! 🐉
> Extremely work-in-progress

## Development

### Prerequisites

- Rust 1.70+ with `clippy` and `rustfmt` components
- Git with submodule support

### Setup

1. Clone the repository and initialize submodules:
   ```bash
   git clone <repo-url>
   cd typefrog
   git submodule update --init --recursive
   ```

2. Install development tools:
   ```bash
   # Run the setup script
   ./scripts/setup-dev.sh
   
   # Or install manually:
   rustup component add clippy rustfmt
   cargo install cargo-llvm-cov
   ```

### Development Commands

Using Make (recommended):
```bash
make help          # Show all available targets
make test          # Run all tests
make lint          # Run clippy linter
make fmt           # Format code with rustfmt
make fmt-check     # Check code formatting
make coverage      # Generate HTML coverage report
make coverage-lcov # Generate lcov coverage report
make check         # Run all checks (fmt, lint, test)
make ci            # Run full CI pipeline locally
```

Using Cargo directly:
```bash
# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Testing with coverage
cargo llvm-cov test --all-features --workspace

# Generate coverage report (HTML)
cargo llvm-cov --all-features --workspace --html

# Generate coverage report (lcov format)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Format code
cargo fmt
```

### Code Quality

This project enforces high code quality standards:

- **Clippy**: Pedantic and nursery lints enabled with sensible exceptions
- **Coverage**: Comprehensive test coverage tracking via `cargo-llvm-cov`
- **Formatting**: Consistent code formatting with `rustfmt`
- **CI**: Automated checks on all pull requests

#### Clippy Configuration

The project uses strict clippy lints defined in `Cargo.toml`:
- Pedantic and nursery lints enabled by default
- `unsafe_code` is forbidden
- `unwrap_used` is denied (use proper error handling)
- `panic` is denied in production code

See `clippy.toml` for additional configuration.

#### Coverage Reports

Coverage reports are generated in multiple formats:
- **HTML**: `target/llvm-cov/html/index.html` (open in browser)
- **LCOV**: `lcov.info` (for CI/tooling integration)

### CI/CD

The project includes GitHub Actions workflows for:
- Running tests with coverage reporting
- Clippy linting with zero warnings policy
- Code formatting checks
- Coverage upload to Codecov

### Troubleshooting

**Submodule issues**: If git submodules fail to initialize:
```bash
# Retry with timeout
timeout 120s git submodule update --init --recursive

# Or initialize manually if needed
git submodule init
git submodule update
```

**Build failures**: Ensure all dependencies are available:
```bash
cargo clean
cargo build
```
