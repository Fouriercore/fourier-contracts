# Fourier Registry Contract

This is the canonical on-chain database of smart contract reputation records for the Fourier ecosystem. It serves as the decentralized source of truth for contract security statuses, risk levels, and associated verification metadata on the Stellar Soroban network.

## Features

- **Immutable Reputation Records**: Once verified off-chain, trust profiles are recorded on-chain, preventing falsification.
- **O(1) Pagination**: Implements a swap-and-pop list indexing scheme allowing wallets, explorers, and apps to paginated-query registry records.
- **Access Control & Governance Ready**: Fully customizable admin and ownership management, ready for DAO/multisig governance handovers.
- **Custom Event Emission**: Emits standard events for registration, updates, and removals, enabling indexing via Mercury, Zeph, or other indexers.

## Interface

### Functions

- `initialize(env: Env, admin: Address)`: Set the contract administrator.
- `register_contract(...)`: Registers a new contract reputation record. (Admin only)
- `update_reputation(...)`: Updates an existing contract reputation record. (Admin only)
- `remove_contract(...)`: Removes a contract record from the registry. (Admin only)
- `get_reputation(env: Env, contract: Address) -> Option<ContractRecord>`: Retrieves a contract's reputation record.
- `is_verified(env: Env, contract: Address) -> bool`: Checks if a contract is verified.
- `list_contracts(env: Env, offset: u32, limit: u32) -> Vec<ContractRecord>`: Paginated lookup of registered contracts.
- `version(env: Env) -> String`: Returns contract semantic version.

## Development

Run tests:
```bash
cargo test -p fourier-registry
```
