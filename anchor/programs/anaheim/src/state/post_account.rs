use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
  pub author: Pubkey,
  pub timestamp: i64,
  pub content: String,
  pub vote_count: u64,
}

#[account]
pub struct UserVoteMarker {
  pub voted: bool,
}
