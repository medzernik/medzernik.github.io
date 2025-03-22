#!/usr/bin/env bash
set -euo pipefail

echo "Installing dependencies via Cargo..."

# Install trunk (WASM bundler)
echo "Installing trunk..."
cargo install trunk

# Install stylance (CSS in Rust)
echo "Installing stylance..."
cargo install stylance-cli

# Install bacon (background rust code checker)
echo "Installing bacon..."
cargo install bacon

# Install leptosfmt
echo "Installing leptosfmt..."
cargo install leptosfmt

echo "All dependencies installed successfully!"