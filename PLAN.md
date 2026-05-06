# Build Plan — Stellar On-Chain Reputation System

This document outlines how the project will be built, phase by phase.
Each phase maps directly to a set of GitHub issues for contributors.

---

## Phase 1 — Core Contract

The foundation. Everything else depends on this.

### 1.1 Contract initialisation & admin auth
- Implement `initialize(env, admin)` — store admin address in instance storage, panic if already set.
- Implement `set_admin(env, new_admin)` — require current admin auth, rotate the key.

### 1.2 Scoring formula
- Implement `compute_score(tx_count, lp_count, gov_count) -> u32`.
- Weighted formula: tx 40 %, LP 35 %, governance 25 %. Max score: 1000.
- Each signal capped at a reference ceiling before weighting.

### 1.3 Score storage & retrieval
- Implement `set_score` — require admin auth, call `compute_score`, persist `ReputationScore` to persistent storage, emit a contract event.
- Implement `get_score` — read from persistent storage, return `Option<ReputationScore>`.

### 1.4 Contract tests
- Unit tests for `compute_score` (zero inputs, max inputs, mid-range).
- Integration tests using `soroban_sdk::testutils`: round-trip set/get, non-admin rejection, double-init panic, unscored wallet returns None.

---

## Phase 2 — Horizon Indexer (TypeScript)

Reads raw on-chain activity for a wallet and produces the counts the contract needs.

### 2.1 Transaction history fetch
- Implement `fetchWalletActivity` in `sdk/src/indexer.ts`.
- Connect to Horizon, fetch transaction history with pagination (handle >200 txs).

### 2.2 Signal extraction
- For each transaction, fetch its operations.
- Count `liquidity_pool_deposit` and `liquidity_pool_withdraw` ops → `lpCount`.
- Count `invoke_host_function` calls to a configurable governance contract allowlist → `govCount`.
- Return `{ txCount, lpCount, govCount }`.

### 2.3 Indexer tests
- Unit tests with mocked Horizon responses.
- Edge cases: wallet with no transactions, wallet with only LP activity.

---

## Phase 3 — Soroban RPC Client (TypeScript)

Bridges the indexer output to the on-chain contract.

### 3.1 `getScore(walletAddress)`
- Build and simulate a `get_score` transaction via `SorobanRpc`.
- Decode the returned `Option<ReputationScore>` ScVal.
- Return `null` for `None`, mapped `ReputationScore` for `Some`.

### 3.2 `setScore(adminKeypair, wallet, counts)`
- Build, simulate, prepare, sign, and send a `set_score` transaction.
- Return the transaction hash.

### 3.3 Client tests
- Unit tests with mocked RPC responses.
- Test `getScore` returns null for an unscored wallet.

---

## Phase 4 — CLI Update Script

Ties Phase 2 and Phase 3 together into a runnable script.

### 4.1 Implement `scripts/update-score.ts`
- Validate `argv` (wallet address) and env vars (`ADMIN_SECRET`, `CONTRACT_ID`).
- Call `fetchWalletActivity(wallet)`.
- Call `testnetClient(contractId).setScore(adminKeypair, wallet, ...activity)`.
- Print the transaction hash.

---

## Phase 5 — Deployment & Integration

### 5.1 Deploy to testnet
- Write a deployment script using Stellar CLI.
- Document the deployed contract ID.

### 5.2 End-to-end test
- Run `update-score.ts` against a real testnet wallet.
- Verify the score is readable on-chain via `get_score`.

### 5.3 Integration example
- Add a minimal code snippet showing how a third-party DeFi protocol calls `get_score` from within their own Soroban contract.

---

## Phase 6 — Extensions (post-MVP)

These are stretch goals, each suitable as a standalone bounty issue.

| Feature | Description |
|---------|-------------|
| Score decay | Reduce score over ledgers of inactivity |
| Third-party attestations | Allow trusted addresses to boost/flag a wallet's score |
| Pluggable signals | Make scoring weights configurable per-deployment |
| REST API | Wrap the SDK in an HTTP server for off-chain consumers |
| Frontend dashboard | Simple UI to look up any wallet's reputation score |

---

## Issue Creation Guide

When creating a GitHub issue for a phase item, use this structure:

- **Title:** concise, e.g. `[Phase 1.2] Implement compute_score formula`
- **Label:** complexity (`trivial` / `medium` / `high`)
- **Body:** link back to the relevant section in this document, list acceptance criteria

### Complexity mapping

| Phase item | Complexity |
|------------|------------|
| 1.1 Admin auth | Medium |
| 1.2 Scoring formula | Medium |
| 1.3 Score storage | Medium |
| 1.4 Contract tests | Trivial–Medium |
| 2.1–2.2 Indexer | Medium |
| 2.3 Indexer tests | Trivial |
| 3.1–3.2 RPC client | High |
| 3.3 Client tests | Trivial |
| 4.1 CLI script | Trivial |
| 5.1–5.2 Deploy & E2E | Medium |
| 5.3 Integration example | Medium |
| Phase 6 items | High |
