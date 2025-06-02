use anchor_lang::prelude::*;
pub mod create_account;
pub mod create_post;
pub mod create_user;
pub mod initialize;
pub(crate) mod vote_post;
pub(crate) mod use_anaheim;
mod update;
mod decrement;

use anchor_lang::system_program::System;
use anchor_lang::Accounts;
use anchor_lang::prelude::{Account, Program, Signer};
use crate::state::post_account::PostAccount;
pub use crate::close::CloseAnaheim;
pub use crate::instructions::vote_post::*;

pub use create_user::CreateUser;
pub use initialize::Initialize;
use crate::constants::MAX_CONTENT_LENGTH;

#[derive(Accounts)]
pub struct CreatePost<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + 4 + MAX_CONTENT_LENGTH + 64 + 8 + 4
  )]

  pub post: Account<'info, PostAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}
