use anchor_lang::prelude::*;
use crate::state::post_account::PostAccount;
use std::str;

#[derive(Accounts)]
pub struct ClosePost<'info> {
  #[account(mut, close = user)]
  pub post_account: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

pub fn close_post(ctx: Context<ClosePost>) -> Result<()> {
  // Récupère les octets du contenu
  let content_bytes = &ctx.accounts.post_account.content;

  // Tente de décoder en UTF-8 et nettoie les zéros terminaux
  let content_str = str::from_utf8(content_bytes)
    .unwrap_or("<Invalid UTF-8>")
    .trim_end_matches('\0');

  msg!("Closing post account with content: {}", content_str);

  Ok(())
}
