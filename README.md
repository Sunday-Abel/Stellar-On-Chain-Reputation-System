# Stellar On-Chain Reputation System

A protocol that lets Stellar wallets build a **verifiable, on-chain reputation score** based on transaction history, liquidity provision, and governance participation.

**Problem:** DeFi protocols on Stellar cannot distinguish trustworthy actors from malicious ones. There is no shared, composable reputation layer — every protocol starts from zero.

**Goal:** A Soroban smart contract that stores a weighted reputation score (0–1000) for any wallet, fed by an off-chain indexer that reads Horizon data. Any protocol can query the score on-chain in a single contract call.

> This project is **open for contributions** via the [Stellar Wave Program](https://www.drips.network/wave/stellar) on Drips. The codebase is intentionally left as a skeleton — contributors implement the logic, earn Points, and get rewarded.

---

## Architecture

```
Horizon API
    │  (tx history, LP ops, governance calls)
    ▼
sdk/src/indexer.ts  ──── fetchWalletActivity(wallet)
    │
    ▼
scripts/update-score.ts  ──── ties indexer → contract
    │
    ▼
Soroban Contract  ──── set_score / get_score
    │
    ▼
Any DeFi Protocol  ──── reads get_score(wallet) on-chain
```

### Scoring Formula (to be implemented)

| Signal              | Suggested weight | Suggested cap |
|---------------------|-----------------|---------------|
| Transaction count   | 40 %            | 500           |
| LP deposit/withdraw | 35 %            | 100           |
| Governance votes    | 25 %            | 50            |
| **Total**           |                 | **1000**      |

---

## Repository Layout

```
stellar-reputation/
├── contracts/reputation/        # Soroban smart contract (Rust)
│   └── src/lib.rs               # ← skeleton, all logic is TODO
├── sdk/src/
│   ├── indexer.ts               # ← skeleton: Horizon activity reader
│   ├── client.ts                # ← skeleton: Soroban contract caller
│   └── index.ts
├── scripts/
│   └── update-score.ts          # ← skeleton: CLI entry point
├── Cargo.toml                   # Rust workspace
└── sdk/package.json
```

---

## Contributing via Drips Wave

This project is part of the **[Stellar Wave Program](https://www.drips.network/wave/stellar)** — a monthly 7-day sprint where contributors fix issues and earn on-chain rewards.

### How to participate

1. Browse open issues in this repo labelled `Stellar Wave`.
2. Log in at [drips.network/wave](https://www.drips.network/wave) with your GitHub account.
3. Apply to an issue, get assigned by the maintainer, submit a PR.
4. Earn Points → converted to on-chain rewards after the Wave ends.

### Contribution areas

| Area | Files | Complexity |
|------|-------|------------|
| Scoring formula | `contracts/reputation/src/lib.rs` → `compute_score` | Medium |
| Contract storage & auth | `lib.rs` → `initialize`, `set_score`, `get_score`, `set_admin` | Medium |
| Contract tests | `lib.rs` → `mod tests` | Trivial–Medium |
| Horizon indexer | `sdk/src/indexer.ts` | Medium |
| Soroban client | `sdk/src/client.ts` | Medium–High |
| CLI script | `scripts/update-score.ts` | Trivial |
| Governance allowlist | `sdk/src/indexer.ts` | Medium |
| Score decay logic | `contracts/reputation/src/lib.rs` | High |
| REST API wrapper | new file | High |

### Prerequisites for contributors

- [Rust + Cargo](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)
- Node.js ≥ 18

---

## Roadmap

- [ ] Core scoring contract (storage, auth, formula)
- [ ] Horizon indexer (tx, LP, governance signal extraction)
- [ ] Soroban RPC client (read/write scores)
- [ ] CLI update script
- [ ] Score decay over inactivity
- [ ] On-chain attestations from trusted third parties
- [ ] Multi-signal extensibility (pluggable scoring modules)
- [ ] REST API wrapper for off-chain consumers

---

## License

MIT
