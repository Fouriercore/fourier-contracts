# Fourier Contracts: Soroban Reputation Registry

Core Soroban smart contracts powering the Fourier ecosystem's decentralized on-chain reputation registry.

[![CI Status](https://github.com/Fouriercore/fourier-contracts/workflows/CI/badge.svg)](#continuous-integration)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](rust-toolchain.toml)
[![Soroban](https://img.shields.io/badge/Soroban-v26.1.0-blue.svg)](Cargo.toml)

---

## 🚀 The Vision

Building on Stellar Soroban requires robust, real-time security guards. **Fourier Contracts** provides a decentralized, immutable on-chain source of truth for contract reputation, security audits, and risk levels. 

Rather than executing slow or expensive vulnerability detection on-chain, this registry stores audited security states verified off-chain by the **Fourier Intelligence Layer**. Wallets, decentralized applications (dApps), explorers, and browser extensions query the registry in real-time to check contract reputation status *before* executing user transactions, protecting user assets from warnings, scams, and suspended protocols.

---

## 🧱 Workspace Structure

The repository is organized as a Cargo workspace to support modular expansions (such as future governance, verifier consensus, and oracle contracts):

```text
fourier-contracts/
├── .github/             # GitHub CI/CD workflows and PR templates
├── contracts/           # Smart contract crates
│   └── registry/        # The canonical reputation registry database
├── docs/                # Extended protocol, storage, and architecture docs
│   ├── architecture.md  # System layout and modular design
│   ├── protocol.md      # Public endpoints and state machines
│   ├── roadmap.md       # Milestones and project schedule
│   ├── security.md      # Vulnerability disclosure guidelines
│   └── storage.md       # O(1) indexing and TTL policies
├── scripts/             # Native building and testing automation scripts
│   ├── build.sh         # Compiles target contracts to optimized WASM
│   ├── deploy.sh        # Demonstrates deployment via stellar-cli
│   ├── format.sh        # Runs cargo fmt & clippy lint checks
│   └── test.sh          # Runs workspace-wide tests
└── tests/               # Integration test suite
    └── registry.rs      # Comprehensive contract flow tests
```

---

## ⚙️ Getting Started

### Prerequisites

To build, test, and run the Fourier smart contracts, you need:

1. **Rust Toolchain**: Stable release (Rust 1.81.0+ recommended). Install via [rustup](https://rustup.rs/).
2. **WASM Compilation Target**:
   ```bash
   rustup target add wasm32v1-none
   ```
3. **Stellar CLI**: (Recommended) For deploying and interacting with contracts locally or on testnet. Install by following the [Stellar Developer Docs](https://developers.stellar.org/docs/smart-contracts/getting-started/setup).

### Clone the Repository

```bash
git clone https://github.com/Fouriercore/fourier-contracts.git
cd fourier-contracts
```

---

## 🧰 Smart Contract: Registry (`/contracts/registry`)

The `registry` contract maintains the reputation profiles.

### Core Data Structures

The registry stores a strongly typed `ContractRecord` for each contract:

```rust
pub struct ContractRecord {
    pub contract_address: Address,
    pub reputation_status: ReputationStatus,
    pub risk_level: RiskLevel,
    pub risk_score: u32, // 0 (safest) to 100 (highest risk)
    pub reporter: Address,
    pub timestamp: u64,
    pub evidence_hash: BytesN<32>,
    pub version: u32,
    pub last_updated: u64,
}
```

Supported status enums:
- `ReputationStatus`: `Unknown`, `Verified`, `Trusted`, `Warning`, `Scam`, `Suspended`
- `RiskLevel`: `Low`, `Medium`, `High`, `Critical`

### Features
- **Immutable On-Chain Database**: Tamper-proof logs of security profiles.
- **O(1) Pagination Index**: Maintains `ContractAtIndex` and `ContractIndex` maps for fast paginated reads (`list_contracts`) without high-gas loops.
- **Automatic TTL Bumping**: Extends ledger rental periods on every read (`get_reputation`) and write, preventing records from being archived.

---

## 🧪 Development & Scripts

All standard development tasks are wrapped inside the `scripts/` folder and can be run from the repository root:

### 1. Build and Compile
Compiles the contracts to release-ready, optimized WebAssembly targets:
```bash
./scripts/build.sh
```
The compiled output is saved to:
`target/wasm32v1-none/release/fourier_registry.wasm`

### 2. Run Tests
Runs both unit tests and integration tests:
```bash
./scripts/test.sh
```
Or use cargo directly:
```bash
cargo test
```

### 3. Lint & Format
Enforce clean formatting and strict Clippy rules:
```bash
./scripts/format.sh
```

---

## 🛠️ Stellar CLI Integration Guide

Here is a step-by-step example of how developers can interact with the contract using `stellar-cli` on `testnet`.

### 1. Deploy the Contract
First, install the WASM on-chain and deploy a contract instance (substitute `<admin_key>` with your Stellar key identity):

```bash
# Install WASM byte-code
WASM_HASH=$(stellar contract install \
  --network testnet \
  --source <admin_key> \
  --wasm target/wasm32v1-none/release/fourier_registry.wasm)

# Deploy instance
CONTRACT_ID=$(stellar contract deploy \
  --network testnet \
  --source <admin_key> \
  --wasm-hash "$WASM_HASH")

echo "Deployed Contract ID: $CONTRACT_ID"
```

### 2. Initialize the Contract
Set the initial administrator address:

```bash
stellar contract invoke \
  --network testnet \
  --source <admin_key> \
  --id "$CONTRACT_ID" \
  -- \
  initialize \
  --admin "$(stellar keys address <admin_key>)"
```

### 3. Register a Smart Contract's Reputation
Add a new reputation record (requires admin authorization):

```bash
stellar contract invoke \
  --network testnet \
  --source <admin_key> \
  --id "$CONTRACT_ID" \
  -- \
  register_contract \
  --contract "CC111111111111111111111111111111111111111111111111111111" \
  --status "Verified" \
  --risk_level "Low" \
  --risk_score 0 \
  --reporter "$(stellar keys address <admin_key>)" \
  --evidence_hash "0000000000000000000000000000000000000000000000000000000000000000"
```

### 4. Query Reputation
Retrieve the reputation profile of a contract:

```bash
stellar contract invoke \
  --network testnet \
  --id "$CONTRACT_ID" \
  -- \
  get_reputation \
  --contract "CC111111111111111111111111111111111111111111111111111111"
```

### 5. Check Verification Status
Quick check if a contract is safe (returns `true` if `Verified` or `Trusted`):

```bash
stellar contract invoke \
  --network testnet \
  --id "$CONTRACT_ID" \
  -- \
  is_verified \
  --contract "CC111111111111111111111111111111111111111111111111111111"
```

### 6. Paginate Registered Contracts
List registered contract addresses by index offset and page size:

```bash
stellar contract invoke \
  --network testnet \
  --id "$CONTRACT_ID" \
  -- \
  list_contracts \
  --offset 0 \
  --limit 10
```

---

## 📅 Roadmap (2026 - 2027)

- [x] **Phase 1: Core Registry & Scaffolding**
  - Set up workspace and workspace dependency management.
  - Implement the foundational `Registry` contract with O(1) swap-and-pop pagination.
  - Configure unit tests and mock integration test environment.
  - Define Github Actions CI pipeline.
- [ ] **Phase 2: Auditing & Testnet Deployment (Q3 2026)**
  - Conduct internal security audits.
  - Optimize compiler settings to minimize WASM file size below 30KB.
  - Deploy the `Registry` contract to Stellar Testnet.
  - Publish official JS/TS SDK client wrappers.
- [ ] **Phase 3: Governance Transition & Verifier Contract (Q4 2026)**
  - Develop the `Verifier` contract supporting multi-signature verifications.
  - Transfer admin controls of the `Registry` to a multi-sig or DAO governance contract.
- [ ] **Phase 4: Mainnet Launch (Q1 2027)**
  - Run Bug Bounty program for the codebase.
  - Deploy the canonical `Registry` contract to Stellar Mainnet.
  - Onboard threat intelligence teams as trusted reporters.

For more details, see the [Full Roadmap](docs/roadmap.md).

---

## 🤝 Contributing

We welcome community contributions! Here is how you can help:

1. **Bug Reports & Feature Requests**: Open an issue describing the bug or feature.
2. **Pull Requests**:
   - Ensure code is formatted with `./scripts/format.sh`.
   - Ensure all tests pass with `./scripts/test.sh`.
   - Review our [.github/pull_request_template.md](.github/pull_request_template.md) before submitting.

See [docs/security.md](docs/security.md) for reporting security vulnerabilities.

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
