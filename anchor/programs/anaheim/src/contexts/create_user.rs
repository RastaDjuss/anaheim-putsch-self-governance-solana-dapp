// FILE: anchor/programs/anaheim/src/contexts/create_user.rs
use anchor_lang::prelude::*;
use crate::state::UserAccount; // Correct import from the state module

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = authority, space = UserAccount::SIZE)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}