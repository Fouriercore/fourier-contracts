# Fourier Contracts Security Model

Security is of paramount importance to the Fourier ecosystem. This document outlines the security controls, audit strategies, and threat mitigations built into the Fourier smart contracts.

## Defensive Design Patterns

1. **Explicit Authentication Check**:
   Administrative actions require authorization checking using Soroban's native `require_auth` system:
   ```rust
   let admin = access_control::require_admin(&env)?;
   ```
   This verifies the caller is the registered admin address.

2. **Reentrancy Protection**:
   Soroban's architecture prevents reentrancy at the host level for standard cross-contract calls unless explicitly allowed. The Registry contract maintains no external callout loops during state updates, avoiding reentrancy vectors.

3. **Input Sanitation & Strict Assertions**:
   - Scores are validated before persistence to ensure they fit within [0, 100].
   - Self-reporting checks are enforced.
   - Initializers can only be executed once.

4. **Rent & TTL Protection**:
   Expired contracts or storage fields can lead to denial-of-service states. To prevent this, the registry bumps the TTL on every single read or write, ensuring active profiles remain accessible.

---

## Governance & Admin Management

During initial testnet/mainnet launch, the contract admin is configured as a multi-signature account. This mitigates single-point-of-failure vulnerabilities for key management.

As the Fourier ecosystem matures, ownership will be transferred to a governance/DAO contract, removing human control over the reputation database.

---

## Vulnerability Disclosure Policy

If you find a security vulnerability, please do **NOT** open a public issue. Doing so exposes users to potential exploits. 

Instead, report vulnerabilities to `security@fourier.network`. 

Please include:
- A clear description of the vulnerability.
- Step-by-step instructions or test code to reproduce the issue.
- Impact assessment.
