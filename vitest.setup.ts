import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Anaheim } from "./anchor/target/types/anaheim";

// Configure Anchor provider once for all tests
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Expose globally `program` and `anchor` for convenience in specs
declare global {
  var anchor: typeof import("@coral-xyz/anchor");
  var program: Program<Anaheim>;
}
global.program = anchor.workspace.Anaheim as Program<Anaheim>;
global.anchor = anchor;
