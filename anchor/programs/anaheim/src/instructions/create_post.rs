use anchor_lang::prelude::*;
use crate::constants::{MAX_MESSAGE_LENGTH, ErrorCode}; // Import des constantes et messages d'erreur
#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = crate::PostAccount::SIZE)] // Si le chemin est nécessaire
  pub post_account: Account<'info, PostAccount>, // Compte pour stocker les données de post
  #[account(mut)]
  pub user: Signer<'info>, // Utilisateur qui effectue l'action
  pub system_program: Program<'info, System>, // Programme système Solatimestamp: ()timestamp: ()na
}

#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
}

impl PostAccount {
  // Taille du compte pour garantir un stockage minimal sur Solana
  pub const SIZE: usize = 8 + (4 + MAX_MESSAGE_LENGTH) + 32 + 8;
}

pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
  let trimmed_content = content.trim(); // Nettoyage du contenu

  if trimmed_content.is_empty() {
    return err!(ErrorCode::InvalidContent); // Valide si le contenu est vide
  }
  if trimmed_content.len() > MAX_MESSAGE_LENGTH {
    return err!(ErrorCode::ContentTooLong); // Valide si le contenu excède la limite
  }

  let post_account = &mut ctx.accounts.post_account;
  post_account.content = trimmed_content.to_string();
  post_account.author = *ctx.accounts.user.key;
  post_account.timestamp = Clock::get()?.unix_timestamp; // Ajout d’un timestamp

  msg!("Post created. Author: {:?}", post_account.author); // Log d'information
  Ok(())
}
