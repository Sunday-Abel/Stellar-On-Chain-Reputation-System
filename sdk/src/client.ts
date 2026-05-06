import { Keypair, Networks, SorobanRpc } from "@stellar/stellar-sdk";

export interface ReputationScore {
  score: number;
  txCount: number;
  lpCount: number;
  govCount: number;
  lastUpdated: number;
}

export interface ReputationClientConfig {
  contractId: string;
  rpcUrl: string;
  networkPassphrase: string;
}

export class ReputationClient {
  constructor(_config: ReputationClientConfig) {
    // TODO: store config, initialise SorobanRpc.Server and Contract instances
  }

  /**
   * Read the reputation score for a wallet.
   * Returns null if the wallet has not been scored yet.
   *
   * TODO:
   *   1. Build a simulation transaction calling get_score(wallet)
   *   2. Simulate via rpc.simulateTransaction
   *   3. Decode the returned Option<ReputationScore> ScVal
   *   4. Return null for None, or a mapped ReputationScore for Some
   */
  async getScore(_walletAddress: string): Promise<ReputationScore | null> {
    throw new Error("TODO: implement getScore");
  }

  /**
   * Submit a set_score transaction signed by the admin keypair.
   * Returns the transaction hash.
   *
   * TODO:
   *   1. Fetch the admin account from RPC
   *   2. Build a transaction calling set_score(wallet, txCount, lpCount, govCount)
   *   3. Simulate → prepare → sign with adminKeypair → send
   *   4. Return the transaction hash
   */
  async setScore(
    _adminKeypair: Keypair,
    _walletAddress: string,
    _txCount: number,
    _lpCount: number,
    _govCount: number
  ): Promise<string> {
    throw new Error("TODO: implement setScore");
  }
}

// Convenience factory for testnet
export const testnetClient = (contractId: string) =>
  new ReputationClient({
    contractId,
    rpcUrl: "https://soroban-testnet.stellar.org",
    networkPassphrase: Networks.TESTNET,
  });
