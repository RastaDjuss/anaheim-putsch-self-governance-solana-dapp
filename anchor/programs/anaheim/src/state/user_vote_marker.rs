// anchor/programs/anaheim/src/state/user_vote_marker.rs
use anchor_lang::prelude::*;

#[account]
pub struct UserVoteMarker {
  pub has_voted: bool,
  pub is_upvote: bool,
  pub post: Pubkey,
  pub user: Pubkey,
}

impl UserVoteMarker {
  pub const SIZE: usize = 8 + 1 + 1 + 32 + 32;
}
