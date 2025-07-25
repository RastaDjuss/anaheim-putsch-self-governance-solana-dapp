use anchor_lang::prelude::*;
use crate::state::user_vote_marker::UserVoteMarker;

#[derive(Accounts)]
pub struct VotePost<'info> {
    #[account(init_if_needed, payer = authority, space = 8 + 1 + 1)]
    pub vote_marker: Account<'info, UserVoteMarker>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
