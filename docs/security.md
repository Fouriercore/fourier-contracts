# Fourier Contracts --- Bug Bounty & Security Disclosure Policy

Security is paramount to the Fourier ecosystem. We welcome the security community to review our smart contracts and infrastructure. This document defines the scope, reward tiers, and reporting procedures for our bug bounty program.

---

## Vulnerability Disclosure Policy

If you discover a security vulnerability, DO NOT open a public issue. Public disclosure exposes users to potential exploits before a fix can be deployed.

### Reporting Channels

| Method | Details |
|--------|---------|
| Primary Email | security@fourier.network |
| PGP Key Fingerprint | F3E1 2A84 9C76 5D01 B2D8 E4F9 7B0A 1C3D 8E5F 2A90 |
| Response SLA | 48 hours acknowledgment, 7 days initial assessment |

### What to Include in Your Report

- Vulnerability type
- Affected contract/component with file path and line numbers
- Step-by-step reproduction with minimal test code or PoC
- Impact assessment
- Suggested fix (optional)

---

## Scope

### In-Scope

| Category | Examples |
|----------|----------|
| Smart contracts | All contracts in contracts/ directory on main branch |
| Cross-chain bridges | Bridge relay logic, message verification |
| Oracle integrations | Data feed consumption, price freshness validation |
| Access control | Admin functions, role management, multi-sig logic |
| Economic logic | Fee calculations, reward distribution, liquidation math |

### Out-of-Scope

- Issues in dependencies already reported upstream
- Phishing or social engineering attacks against team members
- DoS via gas-griefing (standard blockchain risk)
- Already-known issues in open audit reports
- Speculative issues without concrete exploit path

---

## Reward Tiers

Rewards paid in USDC (or equivalent) to the address in the report.

| Severity | Max Reward | Description |
|----------|-----------|-------------|
| Critical | Up to 0,000 | Direct loss of user funds, protocol insolvency, permanent TVL freeze |
| High | Up to 0,000 | Theft with preconditions, temporary fund lock, severe economic manipulation |
| Medium | Up to ,500 | Logic flaws not directly causing fund loss |
| Low | Up to 00 | Code quality, gas optimizations, informational |

First valid report gets full reward. Duplicates not eligible. Rewards at sole discretion of security team.

---

## Safe Harbor

Security research under this policy is: authorized under applicable laws, exempt from public-disclosure prohibitions, not a TOS violation.

---

## Disclosure Timeline

- Report submitted: Day 0
- Acknowledgment: within 48 hours
- Triage and severity assessment: within 7 days
- Fix deployed: varies by severity
- Public disclosure allowed: after fix is live + 30 days

---

## Contact

- Security: security@fourier.network
- Emergency (critical only): Fourier Discord #security-urgent
- Non-security: hello@fourier.network

*Last updated: July 4, 2026*
