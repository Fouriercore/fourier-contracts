# Fourier Contracts

Core Soroban smart contracts powering the Fourier ecosystem's decentralized on-chain reputation registry.

[![CI Status](https://github.com/Fouriercore/fourier-contracts/workflows/CI/badge.svg)](#continuous-integration)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](rust-toolchain.toml)
[![Soroban](https://img.shields.io/badge/Soroban-v26.1.0-blue.svg)](Cargo.toml)

---

## Overview

Fourier Contracts acts as the foundational, decentralized source of truth for contract reputation, security audits, and risk levels inside the Stellar Soroban network. 

Rather than executing contract scans or vulnerability detection on-chain, this registry stores audited security states verified off-chain by the Fourier intelligence layer. Developers, wallets, explorers, and browser extensions can query the registry to check contract reputations before executing transactions, preventing assets from interacting with warning, scam, or suspended protocols.

---

## Architecture

Designed with modularity and protocol scalability in mind, the repository utilizes a Cargo workspace layout. The first canonical contract is the `Registry` (named `registry`, located under `contracts/registry`), with future slots reserved for verification, DAO governance, and rewards:

```text
fourier-contracts/
├── .github/             # GitHub CI/CD workflows and issue templates
├── contracts/           # Smart contract crates
│   └── registry/        # The canonical reputation registry database
├── docs/                # Extended protocol and storage documentation
├── scripts/             # Native building and testing automation scripts
└── tests/               # Integration test suite
```

For a detailed view of the contract architecture, review the [Architecture Documentation](docs/architecture.md).

---

## Features

- **Immutable On-Chain Database**: Provides tamper-proof records of contract security states.
- **O(1) Pagination Index**: Custom storage indexing mapping allows paginated query scanning (`list_contracts`) efficiently.
- **Auto-Bumping Storage TTL**: Extends ledger rental periods on every query, protecting persistent registry entries from archival.
- **Access Control & Governance Handover**: Owner functions are ready to be transferred to multi-sigs or governance voting systems.

---

## Protocol Design

The registry stores a strongly typed `ContractRecord` for each contract:

| Field | Type | Description |
| :--- | :--- | :--- |
| `contract_address` | `Address` | Target smart contract address. |
| `reputation_status` | `ReputationStatus` | `Unknown`, `Verified`, `Trusted`, `Warning`, `Scam`, `Suspended`. |
| `risk_level` | `RiskLevel` | `Low`, `Medium`, `High`, `Critical`. |
| `risk_score` | `u32` | Rating from `0` (safest) to `100` (highest risk). |
| `reporter` | `Address` | Verification entity address. |
| `timestamp` | `u64` | Registration time in Unix seconds. |
| `evidence_hash` | `BytesN<32>` | Cryptographic hash of audit reports. |
| `version` | `u32` | Schema/record state revision version. |
| `last_updated` | `u64` | Last update time in Unix seconds. |

See the [Protocol Documentation](docs/protocol.md) for full endpoint specifications.

---

## Installation

### Prerequisites
- **Rust Toolchain**: `stable` (Rust 1.81.0+ recommended)
- **Target Target**: `wasm32v1-none`
- **Stellar CLI**: (Optional, for deploying and CLI invocations)

Install the target WASM toolchain:
```bash
rustup target add wasm32v1-none
```

---

## Development

All standard development tasks are wrapped inside the `scripts/` folder:

- **Build**: Compiles the smart contracts to optimized WebAssembly targets.
  ```bash
  ./scripts/build.sh
  ```
- **Test**: Runs the Cargo unit and integration test suite.
  ```bash
  ./scripts/test.sh
  ```
- **Format & Lint**: Runs formatting checks and Clippy.
  ```bash
  ./scripts/format.sh
  ```
- **Deploy**: A mock deployment demonstration tool.
  ```bash
  ./scripts/deploy.sh
  ```

---

## Testing

The integration tests under `tests/registry.rs` cover registry workflows:
```bash
cargo test
```
Tests simulate state modifications, verify authorization barriers, input checks, and exercise the O(1) swap-and-pop pagination lists.

---

## Roadmap

- **Phase 1**: Core registry implementation and test setup (Completed).
- **Phase 2**: Testnet deployment, SDK publication, explorer integration.
- **Phase 3**: Verifier multi-sig contract & DAO governance transition.
- **Phase 4**: Mainnet launch and initial reporter onboarding.

Read the complete [Roadmap](docs/roadmap.md) to learn more.

---

## Contributing

We welcome community contributions. Please review the pull request templates under `.github/` before submitting code changes.

---

## Security

If you discover a security vulnerability, please refer to our [Security Policy](docs/security.md) or submit details directly to `security@fourier.network`.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
