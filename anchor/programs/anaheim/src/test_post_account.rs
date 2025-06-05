use anchor_lang::prelude::*;
use crate::state::post_account::PostAccount;

fn test() {
  let _post = PostAccount {
    author: Pubkey::default(),
    timestamp: 0,
    content: String::from("test"),
    vote_count: 0,
  };
}
