use anchor_lang::prelude::*;
use crate::state::user_account::UserAccount;
use crate::error::ErrorCode;
use crate::constants::MAX_USERNAME_LENGTH;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAccount<'info> {
  #[account(
        init,
        payer = signer,
        space = UserAccount::SIZE,
        seeds = [b"user", signer.key().as_ref()],
        bump
  )]
  pub user_account: Account<'info, UserAccount>,

  #[account(mut)]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<CreateAccount>, name: String) -> Result<()> {
  if name.is_empty() || name.len() > MAX_USERNAME_LENGTH {
    return Err(ErrorCode::InvalidUsername.into());
  }

  let user = &mut ctx.accounts.user_account;
  user.authority = ctx.accounts.signer.key();
  user.username = name;
  Ok(())
}

