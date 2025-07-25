//
use anchor_lang::prelude::*;

pub use crate::error::ErrorCode;
pub use crate::handlers::post_account::PostAccount;
pub use crate::state::user_vote_marker::UserVoteMarker;


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct VotePost<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
        mut,
        has_one = author @ ErrorCode::InvalidAuthority,
  )]
  pub post: Account<'info, PostAccount>,

  pub vote_marker: Account<'info, UserVoteMarker>,

  /// CHECK: Ce champ est juste comparé par clé
  pub author: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}
