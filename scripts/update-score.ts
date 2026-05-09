#!/usr/bin/env node
/**
 * update-score.ts
 *
 * Fetches a wallet's on-chain activity and pushes the reputation score
 * to the Soroban contract on Stellar testnet.
 *
 * Usage:
 *   ADMIN_SECRET=S... CONTRACT_ID=C... npx ts-node scripts/update-score.ts <wallet_address>
 */

import { Keypair } from "@stellar/stellar-sdk";
import { fetchWalletActivity } from "../sdk/src/indexer";
import { testnetClient } from "../sdk/src/client";

async function main() {
  // ── Validate args ────────────────────────────────────────────────────────
  const wallet = process.argv[2];
  if (!wallet) {
    console.error("Usage: update-score.ts <wallet_address>");
    process.exit(1);
  }

  const adminSecret = process.env.ADMIN_SECRET;
  const contractId = process.env.CONTRACT_ID;

  if (!adminSecret) {
    console.error("Missing env var: ADMIN_SECRET");
    process.exit(1);
  }
  if (!contractId) {
    console.error("Missing env var: CONTRACT_ID");
    process.exit(1);
  }

  // ── Fetch activity ───────────────────────────────────────────────────────
  console.log(`Fetching activity for ${wallet}...`);
  const activity = await fetchWalletActivity(wallet);
  console.log(`  tx=${activity.txCount}  lp=${activity.lpCount}  gov=${activity.govCount}`);

  // ── Push score ───────────────────────────────────────────────────────────
  const adminKeypair = Keypair.fromSecret(adminSecret);
  const client = testnetClient(contractId);

  console.log("Submitting score...");
  const txHash = await client.setScore(
    adminKeypair,
    wallet,
    activity.txCount,
    activity.lpCount,
    activity.govCount
  );

  console.log(`Done. tx hash: ${txHash}`);
}

main().catch((err) => {
  console.error(err.message ?? err);
  process.exit(1);
});
