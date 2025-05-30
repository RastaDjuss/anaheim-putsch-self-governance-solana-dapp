// === FILE: programs/anaheim/src/instructions/create_user.rs ===
use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::user_account::UserAccount;

#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(init, payer = user, space = UserAccount::SIZE)]
  pub user_account: Account<'info, UserAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  if username.trim().is_empty() || username.len() > 32 {
    return Err(ErrorCode::InvalidContent.into());
  }
  let account = &mut ctx.accounts.user_account;
  account.username = username;
  account.authority = *ctx.accounts.user.key;
  account.timestamp = Clock::get()?.unix_timestamp;
  Ok(())
}
