// FILE: anchor/programs/anaheim/src/state/post_account.rs

use anchor_lang::prelude::*;

#[account]
pub struct PostAccount {
    pub author: Pubkey,
    pub content: String,
    pub upvotes: u64, // Add this field
    pub downvotes: u64, // Add this field
    pub timestamp: i64,
    pub vote_count: u64,
    pub bump: u8,
}

impl PostAccount {
    pub const SIZE: usize = 8               // discriminator
        + 32                              // author Pubkey
        + 4 + 280                         // content string prefix + max length
        + 8                              // upvotes u64
        + 8                              // downvotes u64
        + 8                              // timestamp i64
        + 8                              // vote_count u64
        + 1;                             // bump u8
}

