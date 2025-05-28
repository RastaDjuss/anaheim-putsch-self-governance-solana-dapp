use anchor_lang::prelude::*;

/// ─── IDENTIFIANT DU PROGRAMME ───────────────────────────────────────────────
declare_id!("CJSrfD5XGt4RkvGYZ8ooCUQfTPbPdZEqfPCo68K1Qxou");

/// ─── CONSTANTES ─────────────────────────────────────────────────────────────
pub const MAX_CONTENT_LENGTH: usize  = 256;
pub const MAX_USERNAME_LENGTH: usize = 32;

/// ─── ERREURS ────────────────────────────────────────────────────────────────
#[error_code]
pub enum ErrorCode {
  #[msg("Content exceeds max allowable length.")]
  ContentTooLong,
  #[msg("Username exceeds max allowable length.")]
  UsernameTooLong,
  #[msg("Content is invalid (empty or whitespace only).")]
  InvalidContent,
}

/// ─── COMPTES STRUCTURÉS ─────────────────────────────────────────────────────
#[account]
pub struct UserAccount {
  pub name: String,
  pub user_authority: Pubkey,
}
impl UserAccount {
  pub const SIZE: usize = 8 + 4 + MAX_USERNAME_LENGTH + 32;
}

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
}
impl PostAccount {
  pub const SIZE: usize = 8 + 4 + MAX_CONTENT_LENGTH + 32 + 8;
}

/// ─── CONTEXTES D'INSTRUCTIONS ───────────────────────────────────────────────
#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(init, payer = authority, space = UserAccount::SIZE)]
  pub user_account: Account<'info, UserAccount>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = PostAccount::SIZE)]
  pub post_account: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

/// ─── PROGRAMME PRINCIPAL ────────────────────────────────────────────────────
#[program]
pub mod anaheim {
  use super::*;

  pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    let trimmed = username.trim();
    if trimmed.is_empty() {
      return err!(ErrorCode::InvalidContent);
    }
    if trimmed.len() > MAX_USERNAME_LENGTH {
      return err!(ErrorCode::UsernameTooLong);
    }

    let user_account = &mut ctx.accounts.user_account;
    user_account.name = trimmed.to_string();
    user_account.user_authority = *ctx.accounts.authority.key;

    Ok(())
  }

  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
      return err!(ErrorCode::InvalidContent);
    }
    if trimmed.len() > MAX_CONTENT_LENGTH {
      return err!(ErrorCode::ContentTooLong);
    }

    let post_account = &mut ctx.accounts.post_account;
    post_account.content = trimmed.to_string();
    post_account.author = *ctx.accounts.user.key;
    post_account.timestamp = Clock::get()?.unix_timestamp;

    msg!("Post created by {:?} at {}", post_account.author, post_account.timestamp);
    Ok(())
  }
}
