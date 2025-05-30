import * as anchor from "@project-serum/anchor";

// Charger le provider local
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Charger l'IDL et initialiser le programme
const idl = require("./target/idl/<program_name>.json");
const programId = new anchor.web3.PublicKey("<PROGRAM_ID>");
const program = new anchor.Program(idl, programId, provider);

// Appeler une instruction du programme
await program.methods
.createJournal("Hello, Solana")
.accounts({
journal: journalAccountPublicKey,
user: wallet.publicKey,
systemProgram: anchor.web3.SystemProgram.programId,
})
.rpc();

console.log("Instruction exécutée avec succès !");
