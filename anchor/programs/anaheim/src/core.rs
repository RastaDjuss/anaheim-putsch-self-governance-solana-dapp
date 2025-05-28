use anchor_lang::prelude::*;

pub mod instruction_set {
    pub mod create_post {
        use super::super::*;
        use anchor_lang::prelude::*;

        pub fn create_post(ctx: Context<super::super::anaheim::CreatePost>, content: String) -> Result<()> {
          if content.len() > MAX_MESSAGE_LENGTH {
            return err!(ErrorCode::ContentTooLong);
          }
            // Initialize the post account
            let post = &mut ctx.accounts.post_account;
            post.content = content;
            post.author = *ctx.accounts.user.key;
            post.timestamp = Clock::get()?.unix_timestamp;

            msg!("Post created successfully: {}", post.content);
            Ok(())
        }
    }
}
use anchor_lang::system_program::ID;
use anchor_lang::Discriminator;
const MAX_MESSAGE_LENGTH: usize = 256;

#[program]
pub mod anaheim {
  use super::*;

  /// Fonction pour créer un post
  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    // Validation : Longueur du contenu
    if content.len() > MAX_MESSAGE_LENGTH {
      return err!(ErrorCode::ContentTooLong);
    }

    // Initialisation des données du post
    let post = &mut ctx.accounts.post_account;
    post.content = content.clone();
    post.author = *ctx.accounts.user.key; // Accès à l'utilisateur
    post.timestamp = Clock::get()?.unix_timestamp;

    msg!("Post created successfully: {}", post.content);
    Ok(())
  }
}
  /// Déclarations des comptes pour l'instruction `create_post`
  #[derive(Accounts)]
  pub struct CreatePost<'info> {
    #[account(init, payer = user, space = PostAccount::SIZE)]
    pub post_account: Account<'info, PostAccount>, // Compte représentant le post créé
    #[account(mut)]
    pub user: Signer<'info>, // Compte utilisateur effectuant la transaction
    pub system_program: Program<'info, System>, // Programme système Solana
  }

  /// Structure pour le compte PostAccount
  #[account]
  pub struct PostAccount {
    pub content: String,
    pub author: Pubkey,
    pub timestamp: i64,
  }

  impl PostAccount {
    pub const SIZE: usize = 8 + (4 + 256) + 32 + 8;
  }

  /// Gestion des codes d'erreur
  #[error_code]
  pub enum ErrorCode {
    #[msg("Invalid username.")]
    InvalidUsername,
    #[msg("Content exceeds max allowable length.")]
    ContentTooLong,
    #[msg("Invalid content.")]
    InvalidContent,
  }
