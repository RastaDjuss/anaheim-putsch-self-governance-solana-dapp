import { Keypair } from "@solana/web3.js";
import { describe, it, expect } from "vitest";

// `program` and `anchor` are globals from vitest.setup.ts

describe("📝 Anaheim Program", () => {
  it("should create a post and fetch its content", async () => {
    // 1) Génère un Keypair pour le compte post et un wallet utilisateur
    const postAccount = Keypair.generate();
    const user = Keypair.generate();

    // 2) Appelle l’instruction createPost
    await program.methods
      .createPost("hello vitest!")
      .accounts({
        postAccount: postAccount.publicKey,
        user: user.publicKey,
      })
      .signers([user, postAccount])
      .rpc();

    // 3) On récupère les données on-chain
    const data = await program.account.postAccount.fetch(postAccount.publicKey);

    // 4a) Le content a bien été enregistré (stocké en bytes)
    const saved = Buffer.from(data.content).toString().trim();
    expect(saved).toBe("hello vitest!");

    // 4b) Si tu as un champ `author`, réactive cette assertion :
    // expect(data.author.toBase58()).toBe(user.publicKey.toBase58());
  });
});
