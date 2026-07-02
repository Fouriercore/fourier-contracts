use soroban_sdk::{contracttype, Address, BytesN};

/// Represents the verification and trust status of a smart contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ReputationStatus {
    /// Initial or unresolved reputation.
    Unknown = 0,
    /// Verified identity/source contract.
    Verified = 1,
    /// High trust contract from ecosystem partners.
    Trusted = 2,
    /// General caution, potential minor risk/unconfirmed reports.
    Warning = 3,
    /// Confirmed malicious behavior, phishing, or exploit.
    Scam = 4,
    /// Suspended for administrative or investigative reasons.
    Suspended = 5,
}

/// Represents the risk severity level of a smart contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum RiskLevel {
    /// Minimal to no risk.
    Low = 0,
    /// Moderate or minor risk warnings.
    Medium = 1,
    /// Significant risk factors identified.
    High = 2,
    /// Severe danger, user interaction strongly discouraged.
    Critical = 3,
}

/// The on-chain database entry for a contract's reputation profile.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ContractRecord {
    /// The target contract address being rated.
    pub contract_address: Address,
    /// Verification and trust status.
    pub reputation_status: ReputationStatus,
    /// Risk level classification.
    pub risk_level: RiskLevel,
    /// Risk score (0 to 100, where 0 is safest and 100 is highest risk).
    pub risk_score: u32,
    /// The off-chain agent or authority that reported this record.
    pub reporter: Address,
    /// Time of the initial registration (in Unix seconds).
    pub timestamp: u64,
    /// IPFS/content hash of the verification evidence or audit reports.
    pub evidence_hash: BytesN<32>,
    /// Record schema version for upgrade-ability.
    pub version: u32,
    /// Timestamp of the last update.
    pub last_updated: u64,
}

/// Type-safe keys used to store data in Soroban storage.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// The administrator address (stores Address in Instance storage).
    Admin,
    /// A contract's reputation record (stores ContractRecord in Persistent storage).
    Record(Address),
    /// Total count of registered contracts (stores u32 in Instance storage).
    TotalContracts,
    /// Mapping from index to contract address (stores Address in Persistent storage).
    ContractAtIndex(u32),
    /// Mapping from contract address to its pagination index (stores u32 in Persistent storage).
    ContractIndex(Address),
}
