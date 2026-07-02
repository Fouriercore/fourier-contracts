# Fourier Contracts Storage Layout & TTL Policy

This document describes how the Fourier Registry contract manages persistent and instance storage on Stellar's Soroban network, including rental costs and pagination optimizations.

## Soroban Storage Types

We utilize Soroban's native storage types selectively based on gas efficiency and lifetime requirements:

1. **Instance Storage**:
   - Stored in the contract instance. Loaded automatically on every invocation.
   - Used for configuration and high-frequency global counters.
   - Keys: `Admin`, `TotalContracts`.

2. **Persistent Storage**:
   - Stored in separate ledger entries. Stays in state as long as rent is paid.
   - Used for entity records and large lists to prevent clogging the instance size.
   - Keys: `Record(Address)`, `ContractAtIndex(u32)`, `ContractIndex(Address)`.

---

## Data Keys Layout

```rust
pub enum DataKey {
    Admin,                      // Instance -> Address
    Record(Address),            // Persistent -> ContractRecord
    TotalContracts,             // Instance -> u32
    ContractAtIndex(u32),       // Persistent -> Address
    ContractIndex(Address),     // Persistent -> u32
}
```

---

## O(1) Swap-and-Pop Pagination Index

Iterating over dynamic storage ranges is not natively supported in Soroban. To support listing registry contracts without high-gas looping, we construct an index:

- We maintain `TotalContracts` (the length of our list).
- We map each index `0..TotalContracts` to an address: `ContractAtIndex(index) -> Address`.
- We map each address to its current index: `ContractIndex(Address) -> index`.

### Removal Algorithm

When removing an address (e.g., `C_target` at index `idx`):
1. Load the last index in the list: `last_idx = TotalContracts - 1`.
2. Load the address of the contract at `last_idx` (say `C_last`).
3. If `C_target` is not the last item:
   - Copy `C_last` into index position `idx`: `ContractAtIndex(idx) = C_last`.
   - Update `C_last`'s index reference to `idx`: `ContractIndex(C_last) = idx`.
4. Delete the storage entry for the old last position: delete `ContractAtIndex(last_idx)`.
5. Delete the index and record mappings for the removed contract: delete `ContractIndex(C_target)` and delete `Record(C_target)`.
6. Decrement `TotalContracts` by 1.

This layout allows registering, updating, removing, and paginating records in O(1) time complexity.

---

## TTL (Time-To-Live) & Rent Policy

To prevent contract data from being archived, we proactively bump key lifetimes during every read/write operation.

### Configured Constants
- **Day in Ledgers**: `17,280` ledgers (assuming a 5-second ledger block time).
- **Bump Threshold**: `7` Days (`120,960` ledgers). If an entry's TTL falls below this, it will be extended.
- **Bump Amount**: `30` Days (`518,400` ledgers). The target lifetime set when bumping.

Whenever `get_reputation` or `register_contract` is called, the contract triggers:
```rust
env.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_AMOUNT);
```
This ensures active records are never archived due to storage expiration.
