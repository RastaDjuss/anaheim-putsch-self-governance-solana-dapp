// anchor/programs/anaheim/src/handlers/vote_post.rs
use anchor_lang::prelude::*;
use crate::contexts::vote_post::VotePost;
use crate::error::ErrorCode;

pub fn handle_vote_post(ctx: Context<VotePost>, upvote: bool) -> Result<()> {
    let marker = &mut ctx.accounts.vote_marker;

    require!(!marker.did_vote, ErrorCode::AlreadyVoted);

    marker.did_vote = true;
    marker.upvote = upvote;

    Ok(())
}
