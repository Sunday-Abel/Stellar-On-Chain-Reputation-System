#!/usr/bin/env node
/**
 * update-score.ts
 *
 * CLI script that ties the indexer and contract client together:
 *   1. Read a wallet address from argv
 *   2. Fetch its on-chain activity via fetchWalletActivity
 *   3. Push the resulting counts to the contract via ReputationClient.setScore
 *
 * Usage:
 *   ADMIN_SECRET=S... CONTRACT_ID=C... npx ts-node scripts/update-score.ts <wallet_address>
 *
 * TODO: implement this script
 *   - Validate argv and required env vars (ADMIN_SECRET, CONTRACT_ID)
 *   - Call fetchWalletActivity(wallet)
 *   - Call testnetClient(contractId).setScore(adminKeypair, wallet, ...activity)
 *   - Print the resulting transaction hash
 */

throw new Error("TODO: implement update-score.ts");
