import * as anchor from '@coral-xyz/anchor';
import { config } from 'dotenv';
import { Keypair } from '@solana/web3.js';
import { describe, it, beforeAll, expect } from 'vitest';
import { Program } from '@coral-xyz/anchor';
import type { Anaheim } from '../anaheim-putsch-govdapp/anchor/target/types/anaheim'; // Généré par Anchor

config(); // Charge les variables d'environnement

describe('Anaheim tests', () => {
  let provider: anchor.AnchorProvider;
  let program: Program<Anaheim>;
  let anaheimKeypair: Keypair;
  let connection: anchor.web3.Connection;

  beforeAll(async () => {
    anaheimKeypair = Keypair.generate();

    try {
      provider = anchor.AnchorProvider.env();
      anchor.setProvider(provider);
      connection = provider.connection;

      const version = await connection.getVersion();
      console.log('✅ Connected to Solana cluster:', version);

      // Vérifie si le compte airdropé existe
      const accountExists = !!(await connection.getAccountInfo(anaheimKeypair.publicKey));
      if (!accountExists) {
        const airdropSig = await connection.requestAirdrop(
          anaheimKeypair.publicKey,
          anchor.web3.LAMPORTS_PER_SOL
        );
        await connection.confirmTransaction(airdropSig, 'processed');
      }

      // Initialisation du programme Anchor
      program = anchor.workspace.Anaheim as Program<Anaheim>;
    } catch (err) {
      console.error('❌ Failed to setup Anaheim test environment:', err);
      throw err;
    }
  });

  it('Initializes Anaheim with count = 0', async () => {
    try {
      const txSignature = await program.methods.initialize().accounts({
        anaheim: anaheimKeypair.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([anaheimKeypair]).rpc();

      if (!txSignature) {
        throw new Error('Transaction signature is null or undefined');
      }

      const timeoutMs = 10000;
      const timeoutPromise = new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error('Timeout while confirming transaction')), timeoutMs)
      );

      await Promise.race([
        connection.confirmTransaction(txSignature, 'processed'),
        timeoutPromise,
      ]);

      console.log('✅ Transaction confirmed:', txSignature);

      // Récupère le compte Anaheim (Anchor gère le décodeur Borsh)
      const accountInfo = await program.account.anaheim.fetch(anaheimKeypair.publicKey);

      expect(accountInfo.count.toNumber()).toBe(0);
    } catch (err) {
      console.error('❌ Test failed:', err);
      throw err instanceof Error
        ? new Error(`Test failed: ${err.message}`)
        : new Error(`Test failed: ${String(err)}`);
    }
  });
});
