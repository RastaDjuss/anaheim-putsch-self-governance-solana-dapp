use anchor_lang::prelude::*;
use crate::state::UserAccount;
use crate::constants::MAX_USERNAME_LENGTH;
use crate::ErrorCode;

#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(init, payer = authority, space = 8 + UserAccount::SIZE)]
  pub user_account: Account<'info, UserAccount>, // Compte utilisateur
  #[account(mut)]
  pub authority: Signer<'info>, // Utilisateur qui paiera les frais
  pub system_program: Program<'info, System>, // Programme système Solana
}

pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  let trimmed_username = username.trim();

  if trimmed_username.is_empty() {
    return err!(ErrorCode::InvalidContent);
  }

  if trimmed_username.len() > MAX_USERNAME_LENGTH {
    return err!(ErrorCode::UsernameTooLong);
  }

  let user_account = &mut ctx.accounts.user_account;
  user_account.name = trimmed_username.to_string();
  user_account.user_authority = *ctx.accounts.authority.key;

  Ok(())
}
