use crate::errors::ContractError;
use crate::types::ReputationStatus;
use soroban_sdk::Address;

/// Validates that the risk score is within the acceptable percentage range [0, 100].
pub fn validate_risk_score(score: u32) -> Result<(), ContractError> {
    if score > 100 {
        return Err(ContractError::InvalidRiskScore);
    }
    Ok(())
}

/// Validates that the reputation status is a recognized value.
pub fn validate_reputation_status(status: ReputationStatus) -> Result<(), ContractError> {
    match status {
        ReputationStatus::Unknown
        | ReputationStatus::Verified
        | ReputationStatus::Trusted
        | ReputationStatus::Warning
        | ReputationStatus::Scam
        | ReputationStatus::Suspended => Ok(()),
    }
}

/// Performs address sanity checks, such as preventing a contract from reporting itself.
pub fn validate_addresses(contract: &Address, reporter: &Address) -> Result<(), ContractError> {
    if contract == reporter {
        return Err(ContractError::InvalidAddress);
    }
    Ok(())
}
