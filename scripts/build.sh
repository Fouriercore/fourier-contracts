#!/bin/bash
set -e

# Change directory to repo root
cd "$(dirname "$0")/.."

echo "========================================="
echo "Building Fourier Smart Contracts..."
echo "========================================="

# Ensure the target wasm toolchain is installed
rustup target add wasm32v1-none

# Run the cargo release build for WASM
cargo build --target wasm32v1-none --release

echo "-----------------------------------------"
echo "Build succeeded! Contract binary location:"
echo "target/wasm32v1-none/release/fourier_registry.wasm"
echo "========================================="
