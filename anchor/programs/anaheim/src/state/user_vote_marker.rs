use anchor_lang::prelude::*;

#[account]
pub struct UserVoteMarker {
  pub user: Pubkey,
  pub post: Pubkey,
  pub has_voted: bool,
}

impl UserVoteMarker {
  pub const SIZE: usize = 8 + 32 + 32 + 1;
}
