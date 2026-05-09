#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

// ── Storage keys ──────────────────────────────────────────────────────────────

const ADMIN: Symbol = symbol_short!("ADMIN");

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

// ── Scoring caps ──────────────────────────────────────────────────────────────

const TX_CAP: u32 = 500;
const LP_CAP: u32 = 100;
const GOV_CAP: u32 = 50;

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    /// One-time setup. Stores the admin address that is allowed to write scores.
    /// Panics if already initialised.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Transfer admin rights to a new address.
    pub fn set_admin(env: Env, new_admin: Address) {
        let current: Address = env.storage().instance().get(&ADMIN).unwrap();
        current.require_auth();
        env.storage().instance().set(&ADMIN, &new_admin);
    }

    /// Write (or overwrite) a reputation score for `wallet`.
    /// Only the admin may call this.
    pub fn set_score(
        env: Env,
        wallet: Address,
        tx_count: u32,
        lp_count: u32,
        gov_count: u32,
    ) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let score = Self::compute_score(tx_count, lp_count, gov_count);
        let entry = ReputationScore {
            score,
            tx_count,
            lp_count,
            gov_count,
            last_updated: env.ledger().sequence(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Score(wallet.clone()), &entry);

        env.events()
            .publish((symbol_short!("score_set"), wallet), score);
    }

    /// Returns the stored ReputationScore for `wallet`, or None if not yet scored.
    pub fn get_score(env: Env, wallet: Address) -> Option<ReputationScore> {
        env.storage()
            .persistent()
            .get(&DataKey::Score(wallet))
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    /// Weighted scoring formula. Returns a value in 0..=1000.
    /// Weights: tx 40 %, lp 35 %, gov 25 %. Each signal capped before weighting.
    fn compute_score(tx_count: u32, lp_count: u32, gov_count: u32) -> u32 {
        let tx = tx_count.min(TX_CAP);
        let lp = lp_count.min(LP_CAP);
        let gov = gov_count.min(GOV_CAP);

        // Scale each to 0–1000 then apply weight (integer arithmetic, no floats)
        let tx_score = tx * 1000 / TX_CAP * 40 / 100;
        let lp_score = lp * 1000 / LP_CAP * 35 / 100;
        let gov_score = gov * 1000 / GOV_CAP * 25 / 100;

        tx_score + lp_score + gov_score
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, ReputationContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, ReputationContract);
        let client = ReputationContractClient::new(&env, &contract_id);
        (env, client)
    }

    // ── compute_score unit tests ──────────────────────────────────────────────

    #[test]
    fn score_zero_inputs() {
        assert_eq!(ReputationContract::compute_score(0, 0, 0), 0);
    }

    #[test]
    fn score_max_inputs() {
        assert_eq!(ReputationContract::compute_score(500, 100, 50), 1000);
    }

    #[test]
    fn score_caps_excess_inputs() {
        // Values above cap should produce same result as cap
        assert_eq!(
            ReputationContract::compute_score(9999, 9999, 9999),
            ReputationContract::compute_score(500, 100, 50)
        );
    }

    #[test]
    fn score_partial_inputs() {
        // Half of each cap → roughly 500
        let s = ReputationContract::compute_score(250, 50, 25);
        assert!(s > 490 && s <= 510, "expected ~500, got {s}");
    }

    // ── Integration tests ─────────────────────────────────────────────────────

    #[test]
    fn initialize_and_set_get_score_roundtrip() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let wallet = Address::generate(&env);

        client.initialize(&admin);
        // The generated client omits `env`; wallet is passed as the first arg to set_score
        client.set_score(&wallet, &500, &100, &50);

        let result = client.get_score(&wallet).unwrap();
        assert_eq!(result.score, 1000);
        assert_eq!(result.tx_count, 500);
        assert_eq!(result.lp_count, 100);
        assert_eq!(result.gov_count, 50);
    }

    #[test]
    fn unscored_wallet_returns_none() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let wallet = Address::generate(&env);

        client.initialize(&admin);
        assert!(client.get_score(&wallet).is_none());
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn double_initialize_panics() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn set_admin_rotates_key() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let new_admin = Address::generate(&env);
        let wallet = Address::generate(&env);

        client.initialize(&admin);
        client.set_admin(&new_admin);

        // new_admin can now write scores (mock_all_auths covers both)
        client.set_score(&wallet, &100, &50, &25);
        assert!(client.get_score(&wallet).is_some());
    }
}
