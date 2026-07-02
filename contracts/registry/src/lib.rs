#![no_std]

mod access_control;
mod errors;
mod events;
mod metadata;
mod storage;
mod types;
mod validation;

pub use crate::errors::ContractError;
pub use crate::types::{ContractRecord, ReputationStatus, RiskLevel};

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

/// The Fourier Contracts Reputation Registry smart contract.
///
/// This contract maintains an on-chain registry of audited and verified contract reputation states.
/// It acts as a decentralized database of security, trust, and risk metrics that can be integrated
/// directly by wallets, browsers, applications, and indexers in the Stellar Soroban ecosystem.
#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Initializes the registry contract and configures the administrator.
    ///
    /// This function must be called exactly once immediately after deployment.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `admin` - The address that will hold administration rights over the registry.
    ///
    /// # Errors
    ///
    /// * `AlreadyExists` - If the registry has already been initialized with an administrator.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        access_control::set_initial_admin(&env, &admin)?;
        Ok(())
    }

    /// Registers a new contract reputation profile.
    ///
    /// This function is restricted to the contract administrator.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `contract` - The address of the contract being rated.
    /// * `status` - The reputation status to assign (e.g. Verified, Trusted, Warning, Scam).
    /// * `risk_level` - The risk severity level (e.g. Low, Medium, High, Critical).
    /// * `risk_score` - The risk score ranging from 0 (safest) to 100 (highest risk).
    /// * `reporter` - The address of the off-chain entity reporting or certifying this record.
    /// * `evidence_hash` - The 32-byte cryptographic hash of the verification data or audit report.
    ///
    /// # Errors
    ///
    /// * `Unauthorized` - If the caller is not the administrator.
    /// * `AlreadyExists` - If a record for the target contract already exists.
    /// * `InvalidRiskScore` - If the risk score is greater than 100.
    /// * `InvalidAddress` - If `contract` and `reporter` are identical.
    pub fn register_contract(
        env: Env,
        contract: Address,
        status: ReputationStatus,
        risk_level: RiskLevel,
        risk_score: u32,
        reporter: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        // Authenticate admin
        access_control::require_admin(&env)?;

        // Validate inputs
        validation::validate_risk_score(risk_score)?;
        validation::validate_reputation_status(status)?;
        validation::validate_addresses(&contract, &reporter)?;

        // Verify record does not already exist
        if storage::get_record(&env, &contract).is_some() {
            return Err(ContractError::AlreadyExists);
        }

        let now = env.ledger().timestamp();
        let record = ContractRecord {
            contract_address: contract.clone(),
            reputation_status: status,
            risk_level,
            risk_score,
            reporter: reporter.clone(),
            timestamp: now,
            evidence_hash,
            version: 1,
            last_updated: now,
        };

        // Write to storage
        storage::set_record(&env, &contract, &record);
        storage::add_contract_to_list(&env, &contract);

        // Emit events
        events::emit_contract_registered(
            &env,
            contract.clone(),
            status,
            risk_level,
            risk_score,
            reporter.clone(),
        );

        if matches!(
            status,
            ReputationStatus::Verified | ReputationStatus::Trusted
        ) {
            events::emit_contract_verified(&env, contract, reporter);
        }

        Ok(())
    }

    /// Updates the reputation profile of an already registered contract.
    ///
    /// This function is restricted to the contract administrator.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `contract` - The address of the contract to update.
    /// * `status` - The new reputation status.
    /// * `risk_level` - The new risk level.
    /// * `risk_score` - The new risk score (0 to 100).
    /// * `reporter` - The address of the authority submitting this update.
    /// * `evidence_hash` - The updated cryptographic hash of the audit evidence.
    ///
    /// # Errors
    ///
    /// * `Unauthorized` - If the caller is not the administrator.
    /// * `NotFound` - If no record exists for the given contract.
    /// * `InvalidRiskScore` - If the risk score is greater than 100.
    /// * `InvalidAddress` - If `contract` and `reporter` are identical.
    pub fn update_reputation(
        env: Env,
        contract: Address,
        status: ReputationStatus,
        risk_level: RiskLevel,
        risk_score: u32,
        reporter: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        // Authenticate admin
        access_control::require_admin(&env)?;

        // Validate inputs
        validation::validate_risk_score(risk_score)?;
        validation::validate_reputation_status(status)?;
        validation::validate_addresses(&contract, &reporter)?;

        // Check that record exists
        let mut record = storage::get_record(&env, &contract).ok_or(ContractError::NotFound)?;

        let now = env.ledger().timestamp();
        record.reputation_status = status;
        record.risk_level = risk_level;
        record.risk_score = risk_score;
        record.reporter = reporter.clone();
        record.evidence_hash = evidence_hash;
        record.version += 1;
        record.last_updated = now;

        // Write update to storage
        storage::set_record(&env, &contract, &record);

        // Emit events
        events::emit_reputation_updated(
            &env,
            contract.clone(),
            status,
            risk_level,
            risk_score,
            reporter.clone(),
        );

        if matches!(
            status,
            ReputationStatus::Verified | ReputationStatus::Trusted
        ) {
            events::emit_contract_verified(&env, contract, reporter);
        }

        Ok(())
    }

    /// Retrieves the reputation profile record for a given contract address.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `contract` - The address of the contract to look up.
    ///
    /// # Returns
    ///
    /// * `Some(ContractRecord)` - The reputation profile details, if found.
    /// * `None` - If no record is registered for the given contract.
    pub fn get_reputation(env: Env, contract: Address) -> Option<ContractRecord> {
        storage::get_record(&env, &contract)
    }

    /// Verifies if a contract is currently marked as verified or trusted.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `contract` - The address of the contract to verify.
    ///
    /// # Returns
    ///
    /// * `true` - If the contract has a `Verified` or `Trusted` status.
    /// * `false` - If the contract is warning, scam, suspended, unknown, or not registered.
    pub fn is_verified(env: Env, contract: Address) -> bool {
        if let Some(record) = storage::get_record(&env, &contract) {
            matches!(
                record.reputation_status,
                ReputationStatus::Verified | ReputationStatus::Trusted
            )
        } else {
            false
        }
    }

    /// Removes a contract reputation profile from the registry.
    ///
    /// This function is restricted to the contract administrator.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `contract` - The address of the contract to remove.
    ///
    /// # Errors
    ///
    /// * `Unauthorized` - If the caller is not the administrator.
    /// * `NotFound` - If no record exists for the given contract.
    pub fn remove_contract(env: Env, contract: Address) -> Result<(), ContractError> {
        // Authenticate admin
        let admin = access_control::require_admin(&env)?;

        // Ensure record exists
        if storage::get_record(&env, &contract).is_none() {
            return Err(ContractError::NotFound);
        }

        // Remove from list and records
        storage::remove_contract_from_list(&env, &contract)?;
        storage::remove_record(&env, &contract);

        // Emit events
        events::emit_contract_removed(&env, contract, admin);

        Ok(())
    }

    /// Performs a paginated query to list all registered contract records.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `offset` - The index to start pagination from.
    /// * `limit` - The maximum number of records to return.
    ///
    /// # Returns
    ///
    /// * `Vec<ContractRecord>` - A host-backed vector containing matching records.
    pub fn list_contracts(env: Env, offset: u32, limit: u32) -> Vec<ContractRecord> {
        storage::list_records(&env, offset, limit)
    }

    /// Returns the semantic version of this contract.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    pub fn version(env: Env) -> String {
        String::from_str(&env, metadata::SEMANTIC_VERSION)
    }

    /// Transfers administrator rights to a new address.
    ///
    /// This function is restricted to the contract administrator.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban execution environment.
    /// * `new_admin` - The address of the new administrator.
    ///
    /// # Errors
    ///
    /// * `Unauthorized` - If the caller is not the current administrator.
    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), ContractError> {
        access_control::transfer_admin(&env, &new_admin)?;
        Ok(())
    }
}
