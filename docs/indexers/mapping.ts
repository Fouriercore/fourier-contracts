/**
 * Zeph / Mercury Event Handler Mappings for Fourier Registry
 */

export interface SorobanContractEvent {
  contractId: string;
  topics: string[];
  data: any;
  ledger: number;
  ledgerClosedAt: string;
  txHash: string;
}

export async function handleContractRegistered(event: SorobanContractEvent): Promise<void> {
  const { contractId, topics, data, ledger, txHash } = event;
  const eventName = topics[0];

  if (eventName !== "ContractRegistered") {
    return;
  }

  // Parse fields emitted by emit_contract_registered
  const contractAddress = data.contract || topics[1];
  const status = data.status;
  const riskLevel = data.risk_level;
  const riskScore = Number(data.risk_score || 0);
  const reporter = data.reporter;

  console.log(`[Indexer] Registered contract ${contractAddress} with score ${riskScore} by ${reporter} at ledger ${ledger}`);
}

export async function handleReputationUpdated(event: SorobanContractEvent): Promise<void> {
  const { contractId, topics, data, ledger } = event;
  if (topics[0] !== "ReputationUpdated") return;

  const contractAddress = data.contract || topics[1];
  const status = data.status;
  const riskLevel = data.risk_level;
  const riskScore = Number(data.risk_score || 0);

  console.log(`[Indexer] Updated reputation for ${contractAddress}: ${status} (${riskLevel}, score: ${riskScore})`);
}
