import { Horizon } from "@stellar/stellar-sdk";

export interface WalletActivity {
  txCount: number;
  lpCount: number;
  govCount: number;
}

/**
 * Reads a wallet's on-chain activity from Horizon and returns the raw
 * counts that feed into the reputation scoring formula.
 *
 * TODO: implement the following steps
 *   1. Connect to Horizon (use the provided horizonUrl)
 *   2. Fetch the wallet's transaction history (handle pagination)
 *   3. For each transaction, fetch its operations
 *   4. Count liquidity_pool_deposit / liquidity_pool_withdraw ops → lpCount
 *   5. Count governance-related invoke_host_function calls → govCount
 *      (contributors should define a governance contract allowlist)
 *   6. Return { txCount, lpCount, govCount }
 */
export async function fetchWalletActivity(
  _walletAddress: string,
  _horizonUrl = "https://horizon.stellar.org"
): Promise<WalletActivity> {
  throw new Error("TODO: implement fetchWalletActivity");
}
