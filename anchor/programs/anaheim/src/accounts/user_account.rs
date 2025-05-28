use anchor_lang::prelude::*;
use crate::constants::MAX_USERNAME_LENGTH;

#[account]
pub struct UserAccount {
  pub name: [u8; 32], // Tableau de 32 octets pour stocker le nom
  pub user_authority: Pubkey,
}

impl UserAccount {
  pub const SIZE: usize = 32 + 32; // Mise à jour de la taille (2 champs de type Pubkey)
}
#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(
        init, // Initializes the user_account during transaction.
        payer = signer, // The signer pays for the new account.
        space = UserAccount::SIZE, // Allocate exact account size.
  )]
  pub user_account: Account<'info, UserAccount>, // Stores user-account state.

  #[account(mut)]
  pub signer: Signer<'info>, // The user creating the account.

  pub system_program: Program<'info, System>, // Required for creating new accounts.
}

pub fn handler(ctx: Context<CreateUser>, name: String) -> Result<()> {
  // Ensure username length does not exceed 64 characters.
  if name.chars().count() > 64 {
    return Err(error!(ErrorCode::UsernameTooLong));

#[error_code]
pub enum ErrorCode {
    #[msg("The username exceeds the maximum length of 64 characters.")]
    UsernameTooLong,
}
  }

  // Access the user_account struct and populate fields.
  let user_account = &mut ctx.accounts.user_account;
  user_account.authority = *ctx.accounts.signer.key; // Assign signer authority.
  user_account.name = name; // Assign provided username.
  user_account.reputation = 0; // Initialize reputation as 0.

  Ok(())
}
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct PostAccount {
  pub owner: Pubkey,
  pub content: String,
}

impl PostAccount {
  // Taille : 8 (discriminator) + 32 (Pubkey) + 4 (prefix String) + 280 (contenu max ?)
  pub const MAX_CONTENT_LENGTH: usize = 280; // Adapte cette valeur à ta limite
  pub const SIZE: usize = 8 + 32 + 4 + Self::MAX_CONTENT_LENGTH;
}

#[account]
#[derive(Debug)]
pub struct UserAccount {
  pub authority: Pubkey,
  pub username: String,
}

impl UserAccount {
  // Taille : 8 (discriminator) + 32 (Pubkey) + 4 (prefix String) + 32 (username max)
  pub const MAX_USERNAME_LENGTH: usize = 32;
  pub const SIZE: usize = 8 + 32 + 4 + Self::MAX_USERNAME_LENGTH;
}
