#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// ── Data types ────────────────────────────────────────────────────────────────

/// Reputation score stored on-chain for a wallet.
#[contracttype]
#[derive(Clone)]
pub struct ReputationScore {
    /// Composite score 0–1000
    pub score: u32,
    pub tx_count: u32,
    pub lp_count: u32,
    pub gov_count: u32,
    /// Ledger sequence of last update
    pub last_updated: u32,
}

#[contracttype]
pub enum DataKey {
    Score(Address),
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    /// One-time setup. Stores the admin address that is allowed to write scores.
    /// TODO: implement initialisation and persist admin to instance storage
    pub fn initialize(_env: Env, _admin: Address) {
        todo!("store admin; panic if already initialised")
    }

    /// Write (or overwrite) a reputation score for `wallet`.
    /// Only the admin (off-chain indexer key) may call this.
    /// TODO: require admin auth, compute score, persist to persistent storage, emit event
    pub fn set_score(
        _env: Env,
        _wallet: Address,
        _tx_count: u32,
        _lp_count: u32,
        _gov_count: u32,
    ) {
        todo!("require_auth, compute_score, storage().persistent().set, emit event")
    }

    /// Returns the stored ReputationScore for `wallet`, or None if not yet scored.
    /// TODO: read from persistent storage and return Option<ReputationScore>
    pub fn get_score(_env: Env, _wallet: Address) -> Option<ReputationScore> {
        todo!("storage().persistent().get(&DataKey::Score(wallet))")
    }

    /// Transfer admin rights to a new address (e.g. rotate the indexer key).
    /// TODO: require current admin auth, overwrite admin in instance storage
    pub fn set_admin(_env: Env, _new_admin: Address) {
        todo!("require_auth, storage().instance().set ADMIN key")
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    /// Weighted scoring formula — must return a value in 0..=1000.
    /// Suggested weights: tx_count 40 %, lp_count 35 %, gov_count 25 %.
    /// Each input should be capped at a reference ceiling before weighting.
    /// TODO: implement and unit-test this function
    fn compute_score(_tx_count: u32, _lp_count: u32, _gov_count: u32) -> u32 {
        todo!("weighted formula with per-signal caps")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    // TODO: test that set_score + get_score round-trips correctly
    // TODO: test that max inputs produce score == 1000
    // TODO: test that an unscored wallet returns None
    // TODO: test that double-initialise panics
    // TODO: test that a non-admin cannot call set_score
}
