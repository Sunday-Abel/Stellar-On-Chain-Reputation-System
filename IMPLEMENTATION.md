# Implementation Notes

This document tracks everything implemented in this project, the decisions made, and how to run it locally.

---

## Project Overview

**Stellar On-Chain Reputation System** — a Soroban smart contract that stores a verifiable reputation score (0–1000) for any Stellar wallet, fed by an off-chain indexer that reads Horizon data. Any DeFi protocol can query the score on-chain in a single contract call.

Built as an open-source contribution project for the **Stellar Wave Program on Drips** ([drips.network/wave/stellar](https://www.drips.network/wave/stellar)).

---

## What Was Implemented

### Phase 1 — Core Contract ✅

**File:** `contracts/reputation/src/lib.rs`

#### Storage layout

| Key | Storage type | Value |
|-----|-------------|-------|
| `ADMIN` (Symbol) | Instance | `Address` |
| `DataKey::Score(Address)` | Persistent | `ReputationScore` |

Instance storage is used for the admin because it lives as long as the contract. Persistent storage is used for scores because each wallet entry needs an independent TTL.

#### Functions

**`initialize(env, admin)`**
- Stores the admin address in instance storage.
- Panics with `"already initialized"` if called a second time.
- No auth required — first caller wins (deploy-time setup).

**`set_admin(env, new_admin)`**
- Requires auth from the current admin.
- Overwrites the stored admin address.
- Used to rotate the off-chain indexer key.

**`set_score(env, wallet, tx_count, lp_count, gov_count)`**
- Requires admin auth.
- Calls `compute_score` to derive the composite score.
- Persists a `ReputationScore` struct keyed by wallet address.
- Emits a `score_set` contract event with the wallet and score value.

**`get_score(env, wallet) -> Option<ReputationScore>`**
- Reads from persistent storage.
- Returns `None` if the wallet has never been scored.

**`compute_score(tx_count, lp_count, gov_count) -> u32` (private)**
- Caps each input at its reference ceiling before weighting.
- Returns a value in `0..=1000`.

#### Scoring formula

```
tx_score  = min(tx_count,  500) / 500  × 400   (40%)
lp_score  = min(lp_count,  100) / 100  × 350   (35%)
gov_score = min(gov_count,  50) /  50  × 250   (25%)

total = tx_score + lp_score + gov_score  →  0..=1000
```

All arithmetic is integer-only (no floats, `no_std` compatible).

#### Tests (8 passing)

| Test | Type | Covers |
|------|------|--------|
| `score_zero_inputs` | Unit | `compute_score(0,0,0) == 0` |
| `score_max_inputs` | Unit | `compute_score(500,100,50) == 1000` |
| `score_caps_excess_inputs` | Unit | Values above cap == values at cap |
| `score_partial_inputs` | Unit | Mid-range inputs produce ~500 |
| `initialize_and_set_get_score_roundtrip` | Integration | Full set → get cycle |
| `unscored_wallet_returns_none` | Integration | `get_score` returns `None` |
| `double_initialize_panics` | Integration | Second `initialize` panics |
| `set_admin_rotates_key` | Integration | New admin can write scores |

---

### Phase 4 — CLI Script ✅

**File:** `scripts/update-score.ts`

Wires the indexer and contract client into a single runnable command.

**Flow:**
1. Validates `<wallet_address>` arg and `ADMIN_SECRET` / `CONTRACT_ID` env vars — exits with a clear message if anything is missing.
2. Calls `fetchWalletActivity(wallet)` → logs the raw counts.
3. Calls `testnetClient(contractId).setScore(adminKeypair, wallet, ...)` → logs the tx hash.

**Usage:**
```sh
ADMIN_SECRET=S... CONTRACT_ID=C... npx ts-node scripts/update-score.ts <wallet_address>
```

> The CLI is fully wired. It will work end-to-end once Phases 2 and 3 are implemented by contributors.

---

## Open for Contributors (Wave Bounty Issues)

| Phase | File | Description |
|-------|------|-------------|
| 2 | `sdk/src/indexer.ts` | Fetch wallet activity from Horizon API |
| 3 | `sdk/src/client.ts` | Soroban RPC client (read/write scores) |
| 5 | — | Deploy to testnet + end-to-end test |
| 6 | — | Extensions: score decay, attestations, REST API |

See [`issues.md`](./issues.md) and [`PLAN.md`](./PLAN.md) for full acceptance criteria per issue.

---

## Local Development Setup (Windows)

### Prerequisites

- Rust + Cargo
- Node.js ≥ 18

### Step 1 — Switch Rust to the GNU toolchain

The default Windows Rust install uses MSVC, which requires Visual Studio Build Tools. The GNU toolchain is lighter and works with MinGW.

```powershell
# Check what's installed
rustup toolchain list

# Set GNU as default
rustup default stable-x86_64-pc-windows-gnu

# Add the wasm target
rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-pc-windows-gnu
```

### Step 2 — Install MinGW (provides gcc, dlltool, ld)

The GNU toolchain needs `gcc` and `dlltool` to link test binaries on Windows. Install via winget:

```powershell
winget install --id MSYS2.MSYS2 --silent
```

Then install the MinGW-w64 GCC toolchain inside MSYS2:

```powershell
C:\msys64\usr\bin\bash.exe -lc "pacman -S --noconfirm mingw-w64-x86_64-gcc"
```

### Step 3 — Add MinGW to system PATH (permanent)

Run this in PowerShell **as Administrator**, then restart your terminal:

```powershell
[System.Environment]::SetEnvironmentVariable("PATH", "C:\msys64\mingw64\bin;" + [System.Environment]::GetEnvironmentVariable("PATH", "Machine"), "Machine")
```

### Step 4 — Verify

```powershell
cargo check   # should finish with: Finished `dev` profile
cargo test    # should show: 8 passed; 0 failed
```

### SDK

```powershell
cd sdk
npm install
npm run build
```

---

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on every push and PR:
- `cargo check` — verifies the contract compiles
- `npm run build` — verifies the TypeScript SDK compiles

CI runs on Ubuntu so none of the Windows toolchain setup above applies there.

---

## Why GNU over MSVC?

| | MSVC | GNU (MinGW) |
|---|---|---|
| Requires | Visual Studio Build Tools (~4 GB) | MSYS2 + MinGW (~550 MB) |
| Install complexity | High | Low |
| Works for this project | Yes | Yes |

GNU is the lighter, faster option for a Soroban/Rust project with no Windows-specific dependencies.
