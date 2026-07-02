use crate::errors::ContractError;
use crate::types::{ContractRecord, DataKey};
use soroban_sdk::{Address, Env, Vec};

// Constants for TTL management (measured in ledgers)
// Standard limits: ~7 days threshold, ~30 days extension
const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS;
const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;

/// Extends the TTL of a key in persistent storage.
fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, BUMP_THRESHOLD, BUMP_AMOUNT);
}

/// Extends the TTL of all instance storage keys.
fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(BUMP_THRESHOLD, BUMP_AMOUNT);
}

// --- Admin Storage ---

/// Gets the administrator address if set.
pub fn get_admin(env: &Env) -> Option<Address> {
    extend_instance_ttl(env);
    env.storage().instance().get(&DataKey::Admin)
}

/// Sets the administrator address.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    extend_instance_ttl(env);
}

/// Checks if an administrator address is currently configured.
pub fn has_admin(env: &Env) -> bool {
    extend_instance_ttl(env);
    env.storage().instance().has(&DataKey::Admin)
}

// --- Total Count Storage ---

/// Gets the total number of registered contracts.
pub fn get_total_contracts(env: &Env) -> u32 {
    extend_instance_ttl(env);
    env.storage()
        .instance()
        .get(&DataKey::TotalContracts)
        .unwrap_or(0)
}

/// Sets the total number of registered contracts.
fn set_total_contracts(env: &Env, total: u32) {
    env.storage()
        .instance()
        .set(&DataKey::TotalContracts, &total);
    extend_instance_ttl(env);
}

// --- Reputation Record Storage ---

/// Gets the contract record for a given contract address.
pub fn get_record(env: &Env, contract: &Address) -> Option<ContractRecord> {
    let key = DataKey::Record(contract.clone());
    let record: Option<ContractRecord> = env.storage().persistent().get(&key);
    if record.is_some() {
        extend_persistent_ttl(env, &key);
    }
    record
}

/// Stores or updates the reputation record for a contract.
pub fn set_record(env: &Env, contract: &Address, record: &ContractRecord) {
    let key = DataKey::Record(contract.clone());
    env.storage().persistent().set(&key, record);
    extend_persistent_ttl(env, &key);
}

/// Removes a contract reputation record.
pub fn remove_record(env: &Env, contract: &Address) {
    let key = DataKey::Record(contract.clone());
    env.storage().persistent().remove(&key);
}

// --- List Indexing Storage ---

/// Adds a contract to the paginated list tracking.
pub fn add_contract_to_list(env: &Env, contract: &Address) {
    let mut total = get_total_contracts(env);

    let index_key = DataKey::ContractAtIndex(total);
    let contract_key = DataKey::ContractIndex(contract.clone());

    env.storage().persistent().set(&index_key, contract);
    env.storage().persistent().set(&contract_key, &total);

    extend_persistent_ttl(env, &index_key);
    extend_persistent_ttl(env, &contract_key);

    total += 1;
    set_total_contracts(env, total);
}

/// Removes a contract from the paginated list tracking using O(1) swap-and-pop.
pub fn remove_contract_from_list(env: &Env, contract: &Address) -> Result<(), ContractError> {
    let total = get_total_contracts(env);
    if total == 0 {
        return Err(ContractError::StorageFailure);
    }

    let contract_key = DataKey::ContractIndex(contract.clone());
    let idx: u32 = env
        .storage()
        .persistent()
        .get(&contract_key)
        .ok_or(ContractError::NotFound)?;

    let last_idx = total - 1;
    if idx != last_idx {
        // Retrieve the last contract address
        let last_index_key = DataKey::ContractAtIndex(last_idx);
        let last_contract: Address = env
            .storage()
            .persistent()
            .get(&last_index_key)
            .ok_or(ContractError::StorageFailure)?;

        // Move the last contract to the index of the removed contract
        let target_index_key = DataKey::ContractAtIndex(idx);
        env.storage()
            .persistent()
            .set(&target_index_key, &last_contract);
        extend_persistent_ttl(env, &target_index_key);

        // Update the moved contract's index mapping
        let swapped_contract_key = DataKey::ContractIndex(last_contract);
        env.storage().persistent().set(&swapped_contract_key, &idx);
        extend_persistent_ttl(env, &swapped_contract_key);
    }

    // Clean up the storage keys for the last index and deleted contract
    let last_index_key = DataKey::ContractAtIndex(last_idx);
    env.storage().persistent().remove(&last_index_key);
    env.storage().persistent().remove(&contract_key);

    set_total_contracts(env, last_idx);
    Ok(())
}

/// Lists a page of contract records using offset and limit parameters.
pub fn list_records(env: &Env, offset: u32, limit: u32) -> Vec<ContractRecord> {
    let total = get_total_contracts(env);
    let mut records = Vec::new(env);

    if offset >= total || limit == 0 {
        return records;
    }

    let end = (offset + limit).min(total);
    for idx in offset..end {
        let index_key = DataKey::ContractAtIndex(idx);
        if let Some(contract) = env
            .storage()
            .persistent()
            .get::<DataKey, Address>(&index_key)
        {
            extend_persistent_ttl(env, &index_key);
            if let Some(record) = get_record(env, &contract) {
                records.push_back(record);
            }
        }
    }

    records
}
