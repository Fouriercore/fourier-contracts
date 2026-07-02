#![cfg(test)]

use fourier_registry::{
    RegistryContract, RegistryContractClient, ReputationStatus, RiskLevel,
};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

fn setup_test_env<'a>() -> (Env, RegistryContractClient<'a>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RegistryContract, ());
    let client = RegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, client, admin)
}

#[test]
fn test_initialize() {
    let (env, client, _admin) = setup_test_env();

    // Verify semantic version matches metadata
    assert_eq!(
        client.version(),
        soroban_sdk::String::from_str(&env, "0.1.0")
    );

    // Initializing again should fail
    let second_admin = Address::generate(&env);
    let result = client.try_initialize(&second_admin);
    assert!(result.is_err());
}

#[test]
fn test_register_and_get_reputation() {
    let (env, client, _admin) = setup_test_env();

    let contract_addr = Address::generate(&env);
    let reporter_addr = Address::generate(&env);
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    client.register_contract(
        &contract_addr,
        &ReputationStatus::Verified,
        &RiskLevel::Low,
        &5,
        &reporter_addr,
        &evidence_hash,
    );

    // Verify record contents
    let record = client.get_reputation(&contract_addr).unwrap();
    assert_eq!(record.contract_address, contract_addr);
    assert_eq!(record.reputation_status, ReputationStatus::Verified);
    assert_eq!(record.risk_level, RiskLevel::Low);
    assert_eq!(record.risk_score, 5);
    assert_eq!(record.reporter, reporter_addr);
    assert_eq!(record.evidence_hash, evidence_hash);
    assert_eq!(record.version, 1);

    // Verify is_verified checks out
    assert!(client.is_verified(&contract_addr));
}

#[test]
fn test_update_reputation() {
    let (env, client, _admin) = setup_test_env();

    let contract_addr = Address::generate(&env);
    let reporter_addr = Address::generate(&env);
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    client.register_contract(
        &contract_addr,
        &ReputationStatus::Verified,
        &RiskLevel::Low,
        &0,
        &reporter_addr,
        &evidence_hash,
    );

    // Update reputation to Warning / Medium risk
    let updated_hash = BytesN::from_array(&env, &[2; 32]);
    client.update_reputation(
        &contract_addr,
        &ReputationStatus::Warning,
        &RiskLevel::Medium,
        &45,
        &reporter_addr,
        &updated_hash,
    );

    // Verify state was modified
    let record = client.get_reputation(&contract_addr).unwrap();
    assert_eq!(record.reputation_status, ReputationStatus::Warning);
    assert_eq!(record.risk_level, RiskLevel::Medium);
    assert_eq!(record.risk_score, 45);
    assert_eq!(record.evidence_hash, updated_hash);
    assert_eq!(record.version, 2);

    // Verification check should now be false (Warning status)
    assert!(!client.is_verified(&contract_addr));
}

#[test]
fn test_remove_contract() {
    let (env, client, _admin) = setup_test_env();

    let contract_addr = Address::generate(&env);
    let reporter_addr = Address::generate(&env);
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    client.register_contract(
        &contract_addr,
        &ReputationStatus::Scam,
        &RiskLevel::Critical,
        &99,
        &reporter_addr,
        &evidence_hash,
    );

    assert!(client.get_reputation(&contract_addr).is_some());

    // Remove contract from registry
    client.remove_contract(&contract_addr);

    // Verify lookup fails
    assert!(client.get_reputation(&contract_addr).is_none());
}

#[test]
fn test_list_contracts() {
    let (env, client, _admin) = setup_test_env();
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    let c1 = Address::generate(&env);
    let c2 = Address::generate(&env);
    let c3 = Address::generate(&env);
    let reporter = Address::generate(&env);

    client.register_contract(&c1, &ReputationStatus::Verified, &RiskLevel::Low, &5, &reporter, &evidence_hash);
    client.register_contract(&c2, &ReputationStatus::Warning, &RiskLevel::Medium, &50, &reporter, &evidence_hash);
    client.register_contract(&c3, &ReputationStatus::Scam, &RiskLevel::Critical, &95, &reporter, &evidence_hash);

    // List all contracts
    let list = client.list_contracts(&0, &10);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get(0).unwrap().contract_address, c1);
    assert_eq!(list.get(1).unwrap().contract_address, c2);
    assert_eq!(list.get(2).unwrap().contract_address, c3);

    // List with pagination offset and limit
    let paginated = client.list_contracts(&1, &1);
    assert_eq!(paginated.len(), 1);
    assert_eq!(paginated.get(0).unwrap().contract_address, c2);

    // Test swap-and-pop by removing middle contract c2
    client.remove_contract(&c2);

    // Remaining list should be c1 and c3 (c3 swapped into c2's index)
    let list_after_removal = client.list_contracts(&0, &10);
    assert_eq!(list_after_removal.len(), 2);
    assert_eq!(list_after_removal.get(0).unwrap().contract_address, c1);
    assert_eq!(list_after_removal.get(1).unwrap().contract_address, c3);
}

#[test]
fn test_validation_rules() {
    let (env, client, _admin) = setup_test_env();

    let contract_addr = Address::generate(&env);
    let reporter_addr = Address::generate(&env);
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    // Invalid Risk Score (> 100)
    let result = client.try_register_contract(
        &contract_addr,
        &ReputationStatus::Verified,
        &RiskLevel::Low,
        &101, // invalid risk score
        &reporter_addr,
        &evidence_hash,
    );
    assert!(result.is_err());

    // Invalid Address (contract tries to report itself)
    let result_addr = client.try_register_contract(
        &contract_addr,
        &ReputationStatus::Verified,
        &RiskLevel::Low,
        &0,
        &contract_addr, // reporter same as contract address
        &evidence_hash,
    );
    assert!(result_addr.is_err());
}

#[test]
fn test_transfer_admin() {
    let (env, client, _admin) = setup_test_env();

    let new_admin = Address::generate(&env);
    client.transfer_admin(&new_admin);

    // Verify transfer by performing an administrative task authorized by new admin
    let contract_addr = Address::generate(&env);
    let reporter_addr = Address::generate(&env);
    let evidence_hash = BytesN::from_array(&env, &[1; 32]);

    client.register_contract(
        &contract_addr,
        &ReputationStatus::Verified,
        &RiskLevel::Low,
        &0,
        &reporter_addr,
        &evidence_hash,
    );

    assert!(client.get_reputation(&contract_addr).is_some());
}
