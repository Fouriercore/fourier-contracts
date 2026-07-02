#!/bin/bash
set -e

# Change directory to repo root
cd "$(dirname "$0")/.."

echo "========================================="
echo "Running Fourier Contract Tests..."
echo "========================================="

cargo test --all

echo "========================================="
echo "All tests passed successfully!"
echo "========================================="
