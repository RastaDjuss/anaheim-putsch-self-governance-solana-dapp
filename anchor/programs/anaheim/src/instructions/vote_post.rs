// FILE: anchor/programs/anaheim/src/instructions/vote_post.rs

use anchor_lang::prelude::*;
use crate::state::user_vote_marker::UserVoteMarker;
use crate::contexts::vote_post::VotePost;
use crate::handlers::post_account::PostAccount;
use crate::error::ErrorCode;

pub fn handler(ctx: Context<VotePost>, _bump: u8, upvote: bool) -> Result<()> {
    let post: &mut Account<PostAccount> = &mut ctx.accounts.post;
    let vote_marker: &mut Account<UserVoteMarker> = &mut ctx.accounts.vote_marker;

    if upvote {
        post.vote_count = post.vote_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
    } else {
        post.vote_count = post.vote_count.saturating_sub(1);
    }

    vote_marker.did_vote = true;
    vote_marker.upvote = upvote;
    vote_marker.post = post.key();
    vote_marker.user = ctx.accounts.user.key();

    Ok(())
}
