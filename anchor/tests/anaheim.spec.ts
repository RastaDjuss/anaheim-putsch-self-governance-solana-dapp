import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import type { Anaheim } from '../../target/types/anaheim';

describe('anaheim', () => {
  // Configure le client Anchor pour utiliser le cluster local.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  if (!provider.wallet) {
    throw new Error('Wallet is not initialized in the AnchorProvider.');
  }

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.Anaheim as Program<Anaheim>;
  const anaheimKeypair = Keypair.generate();

  // Fonction utilitaire pour vérifier le champ `count`
  const fetchAndExpectCount = async (expectedCount: number) => {
    const currentCount = await program.account.anaheim.fetch(anaheimKeypair.publicKey) as unknown as { count: number };
    expect(currentCount.count).toEqual(expectedCount);
  };

  // Test 1: Initialisation
  it('Initialize Anaheim', async () => {
    await program.methods['initialize']!()
      .accounts({
        anaheim: anaheimKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([anaheimKeypair])
      .rpc();

    await fetchAndExpectCount(0);
  });

  // Test 2: Incrémentation
  it('Increment Anaheim', async () => {
    await program.methods['increment']!()
      .accounts({
        anaheim: anaheimKeypair.publicKey,
      })
      .rpc();

    await fetchAndExpectCount(1);
  });

  // Test 3: Incrémentation à nouveau
  it('Increment Anaheim Again', async () => {
    await program.methods['increment']!()
      .accounts({
        anaheim: anaheimKeypair.publicKey,
      })
      .rpc();

    await fetchAndExpectCount(2);
  });

  // Test 4: Décrémentation
  it('Decrement Anaheim', async () => {
    await program.methods['decrement']!()
      .accounts({
        anaheim: anaheimKeypair.publicKey,
      })
      .rpc();

    await fetchAndExpectCount(1);
  });

  // Test 5: Définir une valeur spécifique
  it('Set Anaheim value', async () => {
    await program.methods['set']!(42)
      .accounts({
        anaheim: anaheimKeypair.publicKey,
      })
      .rpc();

    await fetchAndExpectCount(42);
  });

  // Test 6: Fermeture du compte
  it('Close the Anaheim account', async () => {
    await program.methods['close']!()
      .accounts({
        payer: payer.publicKey,
        anaheim: anaheimKeypair.publicKey,
      })
      .rpc();

    const accountInfo = await program.account.anaheim.fetchNullable(anaheimKeypair.publicKey);
    expect(accountInfo).toBeNull();
  });

  // **Test supplémentaire** : Validation d'un contenu trop long pour `createPost`
  it('should fail to create a post with too long content', async () => {
    const tooLongContent = 'x'.repeat(300); // Dépasse la limite définie

    const postKeypair = Keypair.generate(); // Génère une clé pour le post

    try {
      await program.methods['createPost']!(tooLongContent)
        .accounts({
          postAccount: postKeypair.publicKey,
          user: payer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([postKeypair])
        .rpc();

      throw new Error('Expected an error but none was thrown'); // Utile si aucune erreur n'a été levée
    } catch (err) {
      // Vérifie que nous obtenons le code de l'erreur attendue
      if (err instanceof anchor.ProgramError) {
        expect(err.code).toEqual('ContentTooLong');
      } else {
        throw err;
      }
    }
  });
});
