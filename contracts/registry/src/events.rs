use crate::types::{ReputationStatus, RiskLevel};
use soroban_sdk::{contractevent, Address, Env};

/// Event emitted when a new contract is registered.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contractevent]
pub struct ContractRegistered {
    pub contract: Address,
    pub status: ReputationStatus,
    pub risk_level: RiskLevel,
    pub risk_score: u32,
    pub reporter: Address,
}

/// Event emitted when a contract's reputation record is updated.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contractevent]
pub struct ReputationUpdated {
    pub contract: Address,
    pub status: ReputationStatus,
    pub risk_level: RiskLevel,
    pub risk_score: u32,
    pub reporter: Address,
}

/// Event emitted when a contract's status is elevated to Verified or Trusted.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contractevent]
pub struct ContractVerified {
    pub contract: Address,
    pub reporter: Address,
}

/// Event emitted when a contract is removed from the registry.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contractevent]
pub struct ContractRemoved {
    pub contract: Address,
    pub admin: Address,
}

/// Emits a `ContractRegistered` event when a new contract reputation record is saved.
pub fn emit_contract_registered(
    env: &Env,
    contract: Address,
    status: ReputationStatus,
    risk_level: RiskLevel,
    risk_score: u32,
    reporter: Address,
) {
    ContractRegistered {
        contract,
        status,
        risk_level,
        risk_score,
        reporter,
    }
    .publish(env);
}

/// Emits a `ReputationUpdated` event when an existing contract reputation is modified.
pub fn emit_reputation_updated(
    env: &Env,
    contract: Address,
    status: ReputationStatus,
    risk_level: RiskLevel,
    risk_score: u32,
    reporter: Address,
) {
    ReputationUpdated {
        contract,
        status,
        risk_level,
        risk_score,
        reporter,
    }
    .publish(env);
}

/// Emits a `ContractVerified` event specifically when a contract's status is elevated to Verified or Trusted.
pub fn emit_contract_verified(env: &Env, contract: Address, reporter: Address) {
    ContractVerified { contract, reporter }.publish(env);
}

/// Emits a `ContractRemoved` event when a reputation record is removed from the registry.
pub fn emit_contract_removed(env: &Env, contract: Address, admin: Address) {
    ContractRemoved { contract, admin }.publish(env);
}
