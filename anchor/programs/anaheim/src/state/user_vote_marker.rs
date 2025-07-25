// FILE: state/user_vote_marker.rs
use anchor_lang::prelude::*;

#[account]
pub struct UserVoteMarker {
  pub did_vote: bool,
  pub upvote: bool,
}
