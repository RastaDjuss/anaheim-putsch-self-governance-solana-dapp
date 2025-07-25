// anchor/programs/anaheim/src/accounts/post_account

use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub content: String, // tableau fixe, impératif pour la taille prédéfinie dans Solana
  pub author: Pubkey,
  pub timestamp: i64,
  pub vote_count: u32,
}


impl PostAccount {
  pub const SIZE: usize = 8 + 280 + 32 + 8 + 4;
}
