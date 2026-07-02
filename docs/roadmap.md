# Fourier Contracts Protocol Roadmap

This document outlines the milestones for the development, auditing, and deployment of Fourier Contracts.

## Phase 1: Core Registry & Scaffolding (Current)
- [x] Set up workspace and workspace dependency management.
- [x] Implement the foundational `Registry` contract.
- [x] Configure unit tests and mock integration test environment.
- [x] Establish formatting, Clippy linting, and target WASM compilation.
- [x] Define Github Actions CI pipeline.

## Phase 2: Auditing & Testnet Deployment (Q3 2026)
- [ ] Conduct internal security audits and code freeze.
- [ ] Contract optimization (optimizing compiler settings to minimize WASM file size below 30KB).
- [ ] Deploy the `Registry` contract to Stellar Testnet.
- [ ] Publish official JS/TS SDK client wrappers for Web application integration.
- [ ] Launch prototype integrations with wallet extensions and transaction explorers.

## Phase 3: Governance Transition & Verifier Contract (Q4 2026)
- [ ] Develop the `Verifier` contract:
  - Supports multi-signature verifications.
  - Implements consensus aggregation for off-chain rating providers.
- [ ] Deploy governance framework:
  - Transfer admin controls of the `Registry` contract to a multi-sig or DAO governance contract.
  - Implement upgrade voting mechanisms.

## Phase 4: Mainnet Launch (Q1 2027)
- [ ] Run Bug Bounty program for the Fourier Contracts codebase.
- [ ] Deploy the canonical `Registry` contract to Stellar Mainnet.
- [ ] Establish initial ecosystem partners as trusted reporters (e.g. smart contract audit firms, threat intelligence teams).
- [ ] Integrate real-time on-chain trust querying inside consumer wallets.
