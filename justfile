# ==============================================================================
# justfile for kpv automation
# ==============================================================================

set dotenv-load

BIN_NAME := "kpv"
CARGO := "cargo"

# default target
default: help

# Show available recipes
help:
    @echo "Usage: just [recipe]"
    @echo "Available recipes:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

# ==============================================================================
# Environment Setup
# ==============================================================================

# Install toolchain components and warm caches
setup:
    @echo "🚚 Fetching dependencies..."
    @{{CARGO}} fetch --locked || echo "(fetch skipped: lockfile not frozen)"

# ==============================================================================
# Development Commands
# ==============================================================================

# Build a debug binary
build:
    @echo "🏗 Building debug binary..."
    @{{CARGO}} build

# Build a release binary
build-release:
    @echo "🏗 Building release binary..."
    @{{CARGO}} build --release

# Run kpv with arbitrary arguments
run *args:
    @echo "🚀 Running {{BIN_NAME}} {{args}}"
    @{{CARGO}} run -- {{args}}

# ==============================================================================
# CODE QUALITY
# ==============================================================================

# Format code using rustfmt
format:
    @echo "🧹 Formatting Rust sources..."
    @{{CARGO}} fmt

# Format check and lint with clippy
lint:
    @echo "🔍 Ensuring formatting is clean..."
    @{{CARGO}} fmt --check
    @echo "🛡 Running clippy..."
    @{{CARGO}} clippy --all-targets --all-features -- -D warnings

# ==============================================================================
# TESTING
# ==============================================================================

# Run all tests
test:
    @echo "🚀 Running all tests..."
    @RUST_TEST_THREADS=1 {{CARGO}} test --all-targets --all-features

# ==============================================================================
# CLEANUP
# ==============================================================================

# Remove build artifacts and caches
clean:
    @echo "🧽 Cleaning build artifacts..."
    @rm -rf target
    @rm -rf .tmp
    @rm -rf coverage
    @rm -rf dist
    @echo "✅ Cleanup completed"
