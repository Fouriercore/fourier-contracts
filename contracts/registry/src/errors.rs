use soroban_sdk::contracterror;

/// Custom error codes returned by public registry contract operations.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracterror]
#[repr(u32)]
pub enum ContractError {
    /// The contract is already registered in the registry.
    AlreadyExists = 1,
    /// The contract was not found in the registry.
    NotFound = 2,
    /// The caller is not authorized to perform the operation.
    Unauthorized = 3,
    /// The provided reputation status is invalid or unsupported.
    InvalidStatus = 4,
    /// The risk score is outside the allowed bounds (0 - 100).
    InvalidRiskScore = 5,
    /// The contract or reporter address is invalid.
    InvalidAddress = 6,
    /// An internal storage operation failed or state is corrupted.
    StorageFailure = 7,
}
