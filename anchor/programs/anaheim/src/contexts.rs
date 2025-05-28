use anchor_lang::prelude::*;
use anchor_lang::error_code;
use anchor_lang::Result;
use crate::constants::{validate_username};

#[test]
fn test_validate_username_empty() {
  assert_eq!(
    validate_username("   "),
    Err(ErrorCode::InvalidUsername)
  );
}
/// Structure pour créer un post
#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = PostAccount::SIZE)]
  pub post_account: Account<'info, PostAccount>, // Compte pour enregistrer un post
  #[account(mut)]
  pub user: Signer<'info>, // L'utilisateur qui exécute la transaction
  pub system_program: Program<'info, System>, // Programme système Solana
}

/// Structure de compte pour un post
#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
}

impl PostAccount {
  // Taille totale d'un PostAccount
  pub const SIZE: usize = 8 + (4 + MAX_CONTENT_LENGTH) + 32 + 8;

  pub fn validate_username(username: &str) -> Result<()> {
    let trimmed_username = username.trim();
    if trimmed_username.is_empty() {
      return Err(error!(ErrorCode::InvalidUsername)); // Correction ici
    }
    if trimmed_username.len() > MAX_USERNAME_LENGTH {
      return Err(ErrorCode::UsernameTooLong.into());
    }
    Ok(())
  }
}

/// Enumération pour gérer les erreurs
#[derive(Debug, Clone, PartialEq, Eq)]
#[error_code]
pub enum ErrorCode {
  #[msg("Content exceeds max allowable length.")]
  ContentTooLong, // Contenu trop long
  #[msg("The username exceeds the maximum length.")]
  UsernameTooLong, // Nom d'utilisateur trop long
  #[msg("Invalid username.")]
  InvalidUsername, // Nom d'utilisateur vide ou non valide
}

/// Constantes globales pour le projet
pub const MAX_CONTENT_LENGTH: usize = 256; // Longueur max d'un post
pub const MAX_USERNAME_LENGTH: usize = 32; // Longueur max d'un nom d'utilisateur
