.PHONY: help check test build doc clean clippy fmt audit publish

# Default target
help:
	@echo "Available targets:"
	@echo "  check     - Check if the code compiles"
	@echo "  test      - Run tests"
	@echo "  build     - Build the project"
	@echo "  doc       - Build documentation"
	@echo "  clean     - Clean build artifacts"
	@echo "  clippy    - Run clippy linting"
	@echo "  fmt       - Check code formatting"
	@echo "  fmt-fix   - Fix code formatting"
	@echo "  audit     - Run security audit"
	@echo "  publish   - Publish to crates.io"
	@echo "  install   - Install the library locally"

# Check if code compiles
check:
	cargo check

# Run tests
test:
	cargo test

# Build the project
build:
	cargo build

# Build documentation
doc:
	cargo doc --no-deps --open

# Clean build artifacts
clean:
	cargo clean

# Run clippy linting
clippy:
	cargo clippy -- -D warnings

# Check code formatting
fmt:
	cargo fmt -- --check

# Fix code formatting
fmt-fix:
	cargo fmt

# Run security audit
audit:
	cargo audit

# Publish to crates.io
publish:
	cargo publish

# Install locally for testing
install:
	cargo install --path .

# Run all checks
all: check test clippy fmt audit

# Prepare for release
release: clean check test clippy fmt audit build doc
