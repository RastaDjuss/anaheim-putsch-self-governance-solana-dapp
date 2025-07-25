// anchor/programs/anaheim/src/contexts/vote_post.rs
use anchor_lang::prelude::*;
use crate::handlers::post_account::PostAccount;
use crate::state::user_vote_marker::UserVoteMarker;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct VotePost<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub post: Account<'info, PostAccount>, // ✅ Correct struct used

    #[account(
        init_if_needed,
        payer = user,
        space = UserVoteMarker::SIZE,
        seeds = [b"vote_marker", post.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub vote_marker: Account<'info, UserVoteMarker>,

    /// CHECK: validated manually
    pub author: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
