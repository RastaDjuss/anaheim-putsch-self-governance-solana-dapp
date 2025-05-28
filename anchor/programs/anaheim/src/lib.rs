use anchor_lang::prelude::*;

declare_id!("CJSrfD5XGt4RkvGYZ8ooCUQfTPbPdZEqfPCo68K1Qxou");

/// ─── CONSTANTES ─────────────────────────────────────────────────────────────
pub const MAX_CONTENT_LENGTH: usize = 256;
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
#[account] // Définit une table de comptes Anchor.
pub struct UserAccount {
  pub name: String,
  pub user_authority: Pubkey, // L'adresse de l'autorité de ce compte utilisateur.
}
impl UserAccount {
  pub const SIZE: usize = 8 + 4 + MAX_USERNAME_LENGTH + 32; // Calcul de la taille du compte.
}
#[account]
pub struct PostAccount {
  pub content: String,
  pub author: Pubkey,
  pub timestamp: i64,
}

/// ─── CONTEXTES D'INSTRUCTIONS ───────────────────────────────────────────────
#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(init, payer = authority, space = UserAccount::SIZE)]
  pub user_account: Account<'info, UserAccount>,
  #[account(mut)]
  pub authority: Signer<'info>, // Signataire de la transaction.
  pub system_program: Program<'info, System>, // Programme système Solana.
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(init, payer = user, space = PostAccount::SIZE)]
  pub post_account: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>, // L'utilisateur signant la création du post.
  pub system_program: Program<'info, System>,
}

/// ─── PROGRAMME PRINCIPAL ────────────────────────────────────────────────────
#[program]
pub mod anaheim {
  use super::*;

  /// Création d'un utilisateur.
  pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    let trimmed = username.trim(); // Supprime les espaces inutiles.
    if trimmed.is_empty() {
      return err!(ErrorCode::InvalidContent); // Retourne une erreur en cas de contenu invalide.
    }
    if trimmed.len() > MAX_USERNAME_LENGTH {
      return err!(ErrorCode::UsernameTooLong); // Retourne une erreur si le nom est trop long.
    }

    let user_account = &mut ctx.accounts.user_account; // Référence mutable.
    user_account.name = trimmed.to_string();
    user_account.user_authority = *ctx.accounts.authority.key; // Enregistre l'autorité.

    Ok(())
  }

  /// Création d'un post.
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
    post_account.author = *ctx.accounts.user.key; // L'auteur du post.
    post_account.timestamp = Clock::get()?.unix_timestamp; // Obtient l'horodatage.

    msg!(
            "Post created by {:?} at {}",
            post_account.author,
            post_account.timestamp
        ); // Message de log.

    Ok(())
  }
}
