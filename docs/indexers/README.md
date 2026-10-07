# Indexer Configuration Templates (Mercury & Zeph)

This directory provides indexing templates to capture on-chain events emitted by the Fourier Registry contract (`contracts/registry/src/events.rs`) into queryable datastores.

---

## 1. Overview of Contract Events

The Fourier Registry emits the following Soroban events:

| Event Name | Topics | Data Payload | Purpose |
|---|---|---|---|
| `ContractRegistered` | `["ContractRegistered"]` | `{ contract, status, risk_level, risk_score, reporter }` | Triggered upon initial registration |
| `ReputationUpdated` | `["ReputationUpdated"]` | `{ contract, status, risk_level, risk_score, reporter }` | Triggered when score or status is updated |
| `ContractVerified` | `["ContractVerified"]` | `{ contract, reporter }` | Triggered when contract is marked trusted |
| `ContractRemoved` | `["ContractRemoved"]` | `{ contract, admin }` | Triggered upon admin de-registration |

---

## 2. Mercury Indexer Setup

[Mercury](https://mercurydata.app/) enables webhook-based and database-backed event stream subscriptions.

### Files
- `mercury.yaml`: Subscription manifest defining contract event topics and destination webhooks.

### Local Running Instructions

1. Install Mercury CLI:
   ```bash
   npm install -g @mercury-data/cli
   ```

2. Register and validate the configuration:
   ```bash
   mercury subscribe --config docs/indexers/mercury.yaml
   ```

3. Run a local webhook receiver:
   ```bash
   npm run start:webhook-listener
   ```

---

## 3. Zeph Indexer Setup

[Zeph](https://github.com/stellar/zeph) provides GraphQL schema indexing for Soroban.

### Files
- `schema.graphql`: Defines GraphQL entities for `RegisteredContract` and `RegistrationEvent`.
- `mapping.ts`: Event transform logic converting raw Soroban events into indexed entities.

### Local Running Instructions

1. Start local Zeph runner using Docker:
   ```bash
   docker run -d --name zeph-indexer \
     -p 8080:8080 \
     -v $(pwd)/docs/indexers:/workspace \
     zeph/indexer:latest
   ```

2. Inspect GraphQL endpoint at:
   `http://localhost:8080/graphql`
