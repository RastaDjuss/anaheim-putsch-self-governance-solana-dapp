// FILE: state/user_vote_marker.rs
use anchor_lang::prelude::*;

#[account]
pub struct UserVoteMarker {
  pub user: Pubkey,
  pub post: Pubkey,
  pub has_voted: bool,     // ✅ ADD
  pub is_upvote: bool,     // ✅ ADD
  pub bump: u8,
  pub did_vote: bool, // ← Ajouté
  pub upvote: bool,   // ← Ajouté
}

impl UserVoteMarker {
  pub const SIZE: usize = 8 + 32 + 32 + 1 + 1 + 1; // discriminator + 2 pubkeys + bools + bump
}
