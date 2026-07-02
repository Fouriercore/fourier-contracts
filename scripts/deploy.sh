#!/bin/bash
# Mock deploy script demonstrating how to deploy the registry contract using stellar-cli.
set -e

# Change directory to repo root
cd "$(dirname "$0")/.."

# Check if stellar CLI is installed
if ! command -v stellar &> /dev/null
then
    echo "Error: 'stellar' CLI not found."
    echo "Please install it: https://developers.stellar.org/docs/smart-contracts/getting-started/setup"
    exit 1
fi

NETWORK="testnet"
ADMIN_KEY="admin"

echo "========================================="
echo "Deploying Fourier Registry to $NETWORK..."
echo "========================================="

# 1. Install WASM
echo "Step 1: Installing contract WASM on-chain..."
WASM_HASH=$(stellar contract install \
  --network "$NETWORK" \
  --source "$ADMIN_KEY" \
  --wasm target/wasm32v1-none/release/fourier_registry.wasm)

echo "Contract WASM installed successfully with hash: $WASM_HASH"

# 2. Deploy Instance
echo "Step 2: Deploying contract instance..."
CONTRACT_ID=$(stellar contract deploy \
  --network "$NETWORK" \
  --source "$ADMIN_KEY" \
  --wasm-hash "$WASM_HASH")

echo "Contract deployed successfully! Contract ID: $CONTRACT_ID"

# 3. Initialize
echo "Step 3: Initializing registry contract..."
stellar contract invoke \
  --network "$NETWORK" \
  --source "$ADMIN_KEY" \
  --id "$CONTRACT_ID" \
  -- \
  initialize \
  --admin "$(stellar keys address $ADMIN_KEY)"

echo "-----------------------------------------"
echo "Deployment and initialization complete!"
echo "Contract ID: $CONTRACT_ID"
echo "========================================="
