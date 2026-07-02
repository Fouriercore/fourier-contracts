#!/bin/bash
set -e

# Change directory to repo root
cd "$(dirname "$0")/.."

echo "========================================="
echo "Formatting and Linting Fourier Contracts..."
echo "========================================="

echo "1. Checking formatting..."
cargo fmt --all --check

echo "2. Running Clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings

echo "========================================="
echo "Formatting and linting checks passed!"
echo "========================================="
