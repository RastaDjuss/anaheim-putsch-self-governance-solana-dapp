use anchor_lang::prelude::*;

/// Structure pour représenter un post
#[account]
pub struct Post {
  pub content: String, // ou [u8; N]
  pub author: Pubkey,
}

impl Post {
  pub const LEN: usize = 8 + 32 + 280; // exemple : discriminator + pubkey + content (taille fixe)
}
