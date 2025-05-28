mod validate_post_content;
mod error_code;

use std::alloc::System;
use solana_program::account_info::Account;
use anchor_lang::prelude::*;
use anchor_lang::prelude::Accounts;
use anchor_lang::prelude::Signer;
use anchor_lang::prelude::Program;
use anchor_lang::declare_id;
use anchor_lang::system_program;

declare_id!("5MnaAytYXpZGURgzVah9khUwLqx6tqRP4AYZPRdsXEKi");

/// CONSTANTES & CODES D’ERREUR
pub const MAX_MESSAGE_LENGTH: usize = 280;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid content. It cannot be empty.")]
  InvalidContent,
  #[msg("Content is too long.")]
  ContentTooLong,
  #[msg("Failed to validate content.")]
  ValidationError,
}

/// COMPTE POUR LES POSTS
#[account(anchor_lang::prelude)]
pub struct PostAccount {
  pub content: String,   // Contenu
  pub author: Pubkey,    // Clé publique de l'auteur
  pub timestamp: i64,    // Horodatage
}

impl PostAccount {
  /// Taille du compte calculée pour Anchor
  pub const SIZE: usize = 8 + 4 + MAX_MESSAGE_LENGTH + 32 + 8;
}

/// CONTEXTE D’INSTRUCTION : CREATE_POST
#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = PostAccount::SIZE)]
  pub post_account: Account<'info, PostAccount>,

  #[account(mut)]
  pub user: Signer<'info>,

  /// Vérification explicite de l’adresse du programme système
  #[account(address = anchor_lang::system_program::ID)]
  pub system_program: Program<'info, System>,
}

/// PROGRAMME PRINCIPAL
#[program]
mod anaheim {
  use super::*;

  /// Fonction pour créer un post
  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    // Vérification personnalisée de la longueur et du contenu
    validate_post_content(&content).map_err(|validation_error| match validation_error {
      "Content is empty." => ErrorCode::InvalidContent.into(),
      "Content exceeds max allowable length." => ErrorCode::ContentTooLong.into(),
      _ => ErrorCode::ValidationError.into(),
    })?;

    // Remplir le compte avec les informations fournies
    let post_account = &mut ctx.accounts.post_account;
    post_account.content = content;
    post_account.author = *ctx.accounts.user.key;
    post_account.timestamp = Clock::get()?.unix_timestamp;

    Ok(())
  }
}

/// FONCTIONS SUPPLÉMENTAIRES
pub fn validate_post_content(content: &str) -> Result<(), &'static str> {
  let trimmed = content.trim();
  if trimmed.is_empty() {
    return Err("Content is empty.");
  }
  if trimmed.len() > MAX_MESSAGE_LENGTH {
    return Err("Content exceeds max allowable length.");
  }
  Ok(())
}
