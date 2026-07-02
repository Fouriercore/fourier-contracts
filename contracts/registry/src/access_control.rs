use crate::errors::ContractError;
use crate::storage;
use soroban_sdk::{Address, Env};

/// Ensures that the transaction is signed and authorized by the configured administrator.
pub fn require_admin(env: &Env) -> Result<Address, ContractError> {
    let admin = storage::get_admin(env).ok_or(ContractError::Unauthorized)?;
    admin.require_auth();
    Ok(admin)
}

/// Sets the initial administrator. Fails if an administrator is already set.
pub fn set_initial_admin(env: &Env, admin: &Address) -> Result<(), ContractError> {
    if storage::has_admin(env) {
        return Err(ContractError::AlreadyExists);
    }
    storage::set_admin(env, admin);
    Ok(())
}

/// Transers the administrator rights to a new address (e.g. a multi-sig or DAO governance contract).
pub fn transfer_admin(env: &Env, new_admin: &Address) -> Result<(), ContractError> {
    require_admin(env)?;
    storage::set_admin(env, new_admin);
    Ok(())
}
