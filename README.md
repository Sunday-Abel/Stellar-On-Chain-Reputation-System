# Stellar On-Chain Reputation System

A protocol that lets Stellar wallets build a **verifiable, on-chain reputation score** based on transaction history, liquidity provision, and governance participation.

**Problem:** DeFi protocols on Stellar cannot distinguish trustworthy actors from malicious ones. There is no shared, composable reputation layer — every protocol starts from zero.

**Goal:** A Soroban smart contract that stores a weighted reputation score (0–1000) for any wallet, fed by an off-chain indexer that reads Horizon data. Any protocol can query the score on-chain in a single contract call.

> This project is **open for contributions**. See the contribution table below for what remains to be built.

---

## Architecture

```
Horizon API
    │  (tx history, LP ops, governance calls)
    ▼
sdk/src/indexer.ts  ──── fetchWalletActivity(wallet)   ← TODO
    │
    ▼
scripts/update-score.ts  ──── ties indexer → contract  ✅ implemented
    │
    ▼
Soroban Contract  ──── set_score / get_score            ✅ implemented
    │
    ▼
Any DeFi Protocol  ──── reads get_score(wallet) on-chain
```

---

## Scoring Formula

| Signal              | Weight | Cap  | Max contribution |
|---------------------|--------|------|-----------------|
| Transaction count   | 40 %   | 500  | 400             |
| LP deposit/withdraw | 35 %   | 100  | 350             |
| Governance votes    | 25 %   | 50   | 250             |
| **Total**           |        |      | **1000**        |

Each signal is capped at its ceiling before weighting. All arithmetic is integer-only (`no_std` compatible).

```
score = min(tx, 500)/500 × 400
      + min(lp, 100)/100 × 350
      + min(gov, 50)/50  × 250
```

---

## Repository Layout

```
stellar-reputation/
├── contracts/reputation/
│   └── src/lib.rs               # ✅ full contract implementation
├── sdk/src/
│   ├── indexer.ts               # ← TODO: Horizon activity reader
│   ├── client.ts                # ← TODO: Soroban RPC caller
│   └── index.ts
├── scripts/
│   └── update-score.ts          # ✅ CLI entry point (wired, ready to run)
├── IMPLEMENTATION.md            # decisions, formula details, local setup
├── PLAN.md                      # phase-by-phase build plan
├── Cargo.toml                   # Rust workspace
└── sdk/package.json
```

---

## What's Implemented

### Soroban Contract (`contracts/reputation/src/lib.rs`) ✅

| Function | Description |
|----------|-------------|
| `initialize(env, admin)` | One-time setup. Stores admin in instance storage. Panics if called twice. |
| `set_admin(env, new_admin)` | Rotates the admin key. Requires current admin auth. |
| `set_score(env, wallet, tx, lp, gov)` | Admin-gated. Computes and persists score. Emits `score_set` event. |
| `get_score(env, wallet)` | Returns `Option<ReputationScore>`. `None` if wallet not yet scored. |
| `compute_score(tx, lp, gov)` | Private. Weighted formula → `u32` in `0..=1000`. |

**8 tests passing** (4 unit + 4 integration):

| Test | Type |
|------|------|
| `score_zero_inputs` | Unit |
| `score_max_inputs` | Unit |
| `score_caps_excess_inputs` | Unit |
| `score_partial_inputs` | Unit |
| `initialize_and_set_get_score_roundtrip` | Integration |
| `unscored_wallet_returns_none` | Integration |
| `double_initialize_panics` | Integration |
| `set_admin_rotates_key` | Integration |

### CLI Script (`scripts/update-score.ts`) ✅

Wired end-to-end. Will work once the SDK (indexer + client) is implemented.

```sh
ADMIN_SECRET=S... CONTRACT_ID=C... npx ts-node scripts/update-score.ts <wallet_address>
```

---

## Open for Contributors

| Area | File | Status | Complexity |
|------|------|--------|------------|
| Horizon indexer | `sdk/src/indexer.ts` → `fetchWalletActivity` | **TODO** | Medium |
| Soroban RPC client | `sdk/src/client.ts` → `getScore`, `setScore` | **TODO** | Medium–High |
| Governance allowlist | `sdk/src/indexer.ts` | **TODO** | Medium |
| Deploy to testnet | — | **TODO** | Medium |
| Score decay logic | `contracts/reputation/src/lib.rs` | **TODO** | High |
| REST API wrapper | new file | **TODO** | High |

See [`PLAN.md`](./PLAN.md) for full acceptance criteria per phase and [`IMPLEMENTATION.md`](./IMPLEMENTATION.md) for implementation decisions and local setup.

### Prerequisites

- [Rust + Cargo](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)
- Node.js ≥ 18

### Running locally

```sh
# Contract tests
cargo test

# SDK build
cd sdk && npm install && npm run build
```

---

## Roadmap

- [x] Core scoring contract (storage, auth, formula)
- [x] Contract tests (8 passing)
- [x] CLI update script
- [ ] Horizon indexer (tx, LP, governance signal extraction)
- [ ] Soroban RPC client (read/write scores)
- [ ] Deploy to testnet + end-to-end test
- [ ] Score decay over inactivity
- [ ] On-chain attestations from trusted third parties
- [ ] Multi-signal extensibility (pluggable scoring modules)
- [ ] REST API wrapper for off-chain consumers

---

## License

MIT
