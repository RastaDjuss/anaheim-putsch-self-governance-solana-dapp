// anchor/programs/anaheim/src/lib.rs
#![allow(deprecated)]
#![allow(unexpected_cfgs)]
// ─── IMPORTS STRUCTURÉS ─────────────────────────────────────────────
use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub mod contexts;
pub mod handlers;
pub mod constants;
pub mod error;
pub mod utils;
pub mod close;

mod validate_post_content;

// ─── RÉEXPORTATION DES STRUCTS POUR #[program] ─────────────────────────
pub use crate::contexts::initialize::Initialize;
pub use crate::contexts::create_user::CreateUser;
pub use crate::contexts::create_post::CreatePost;
pub use crate::contexts::vote_post::VotePost;
pub use crate::contexts::update::UpdatePost;
pub use crate::instructions::use_anaheim::UseAnaheim;

pub use crate::handlers::create_user::handle_create_user;
pub use crate::handlers::handle_create_post;
pub use crate::handlers::vote_post::handle_vote_post;
pub use crate::handlers::initialize_handler::handle_initialize;
pub use crate::handlers::handle_increment;

// Autres exports d’erreurs ou de structs
pub use error::ErrorCode;
use crate::state::Anaheim;

declare_id!("CHTVq7e9xEFqMf261QhruAmBZsuLCBAEr8NDgcYAnqcV");

pub const ANAHEIM_IDL_ID: Pubkey = Pubkey::new_from_array([
  132, 157, 218, 39, 146, 184, 154, 229, 157, 208, 222, 217, 179, 105, 214, 114,
  145, 251, 14, 120, 48, 169, 34, 96, 132, 73, 172, 248, 93, 142, 25, 203,
]);

// 👇 Déclare le trait manquant
pub trait IdlInstruction {
  fn id() -> Pubkey;
}
pub const MAX_CONTENT_LENGTH: usize = 256;
pub const MAX_USERNAME_LENGTH: usize = 32;

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

#[account]
pub struct AnaheimAccount {
  pub authority: Pubkey,
  pub count: u64,
  pub value: u8,
}
impl AnaheimAccount {
  pub const SIZE: usize = 8 + 32 + 8 + 1;
}


impl IdlInstruction for Anaheim {
  fn id() -> Pubkey {
    ANAHEIM_IDL_ID
  }
}

/// ─── PROGRAMME PRINCIPAL ────────────────────────────────────────────────────
#[program]
pub mod anaheim {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    handle_initialize(ctx)
  }

  pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    handle_create_user(ctx, username)
  }

  pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    handle_create_post(ctx, content)
  }

  pub fn vote_post(ctx: Context<VotePost>, upvote: bool) -> Result<()> {
    handle_vote_post(ctx, upvote)
  }

  pub fn increment(ctx: Context<UseAnaheim>) -> Result<()> {
    handle_increment(ctx)
  }

  pub fn decrement(ctx: Context<UseAnaheim>) -> Result<()> {
    ctx.accounts.anaheim.count -= 1;
    Ok(())
  }

  pub fn set(ctx: Context<UseAnaheim>, value: u64) -> Result<()> {
    ctx.accounts.anaheim.count = value;
    Ok(())
  }

  pub fn close(_ctx: Context<crate::close::close_anaheim::CloseAnaheim>) -> Result<()> {
    Ok(())
  }
}
