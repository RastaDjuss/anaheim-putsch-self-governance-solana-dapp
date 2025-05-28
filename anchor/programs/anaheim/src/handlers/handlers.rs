use anchor_lang::prelude::*;
use crate::state::{UserAccount};
use crate::constants::MAX_CONTENT_LENGTH;

// Erreurs personnalisées
#[error_code]
pub enum ErrorCode {
  #[msg("The provided username is too long.")]
  UsernameTooLong,
  #[msg("Invalid username.")]
  InvalidUsername,
  #[msg("Content exceeds max allowable length.")]
  ContentTooLong,
}

// Gestionnairpub username: ()e d'exemple pour créer un utilisateur
pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  let trimmed_username = username.trim();

  // Validation du pseudo
  if trimmed_username.is_empty() {
    return Err(ErrorCode::InvalidUsername.into());
  }
  if trimmed_username.len() > MAX_CONTENT_LENGTH {
    return Err(ErrorCode::UsernameTooLong.into());
  }

  // Mise à jour du compte utilisateur
  let user_account = &mut ctx.accounts.user_account;
  user_account.owner = *ctx.accounts.authority.key;
  user_account.username = trimmed_username.to_string();

  Ok(())
}

// Contexte pour créer un utilisateur
#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(init, payer = authority, space = UserAccount::SIZE)]
  pub user_account: Account<'info, UserAccount>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
}
