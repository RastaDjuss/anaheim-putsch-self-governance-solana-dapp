use anchor_lang::prelude::*;
const MAX_USERNAME_LENGTH: usize = 32;
use anchor_lang::system_program::ID;

#[account]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserAccount {
  pub name: [u8; MAX_USERNAME_LENGTH], // Tableau fixe pour réduire les allocations dynamiques
  pub user_authority: Pubkey,          // Adresse publique de l'utilisateur
}

impl UserAccount {
  pub const SIZE: usize = 8 + MAX_USERNAME_LENGTH + 32; // Discriminator (8) + Taille maximum du nom (32) + Pubkey (32)
}
