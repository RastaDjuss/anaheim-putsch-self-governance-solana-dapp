use anchor_lang::prelude::*;

use crate::state::post_account::PostAccount; // <- IMPORT DIRECT
pub mod vote_post;
pub mod use_anaheim;
pub mod update;
pub mod decrement;
pub mod create_post;
pub mod create_user;

use anchor_lang::system_program::System;
use anchor_lang::Accounts;
use anchor_lang::prelude::{Account, Program, Signer};
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
