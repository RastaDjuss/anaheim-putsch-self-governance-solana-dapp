// === contexts/increment.rs ===
use anchor_lang::prelude::*;
use crate::state::anaheim_account::AnaheimAccount;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub anaheim: Account<'info, AnaheimAccount>,
    pub authority: Signer<'info>,
}
