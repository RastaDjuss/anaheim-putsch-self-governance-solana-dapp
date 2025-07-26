// FILE: anchor/programs/anaheim/src/instructions/vote_post.rs
use anchor_lang::prelude::*;

use crate::contexts::vote_post::VotePost;
use crate::error::ErrorCode;
use crate::state::post_account::PostAccount;
use crate::state::user_vote_marker::UserVoteMarker;

pub fn handler(ctx: Context<VotePost>, _bump: u8, upvote: bool) -> Result<()> {
    let post: &mut Account<PostAccount> = &mut ctx.accounts.post;
    let vote_marker: &mut Account<UserVoteMarker> = &mut ctx.accounts.vote_marker;

    if upvote {
        post.vote_count = post.vote_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
    } else {
        post.vote_count = post.vote_count.saturating_sub(1);
    }

    vote_marker.user = ctx.accounts.authority.key(); // ✅ CORRIGÉ
    vote_marker.post = post.key();
    vote_marker.is_upvote = upvote;
    vote_marker.has_voted = true;

    Ok(())
}
