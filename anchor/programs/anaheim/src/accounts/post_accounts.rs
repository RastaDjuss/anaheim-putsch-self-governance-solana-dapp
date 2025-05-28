use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct PostAccount {
  pub owner: Pubkey,                       // Adresse publique du créateur
  pub content: [u8; Self::MAX_CONTENT_LENGTH], // Contenu en tableau fixe
}

impl PostAccount {
  // Limite de la taille du contenu
  pub const MAX_CONTENT_LENGTH: usize = 280;

  // Taille totale du compte en octets (discriminator + Pubkey + taille préfixe String + contenu)
  pub const SIZE: usize = 8 + 32 + Self::MAX_CONTENT_LENGTH;
}
