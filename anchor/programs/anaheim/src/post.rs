use anchor_lang::prelude::*;
use anchor_lang::system_program::ID;
/// Structure pour représenter un post
#[account]
pub struct Post {
  pub content: String,     // Contenu du post (max 256 caractères)
  pub author: Pubkey,      // Adresse publique de l'auteur
  pub created_at: i64,     // Timestamp Unix de création
}

impl Post {
  pub const SIZE: usize = 8 + (4 + 256) + 32 + 8;
}
