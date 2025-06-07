use anchor_lang::prelude::*;

use crate::state::anaheim::Anaheim;  // c’est ici que se trouve ta struct #[account] Anaheim
use crate::state::anaheim_account::AnaheimAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    payer = user,
    space = 8 + Anaheim::SIZE,
    )]
    pub anaheim: Account<'info, Anaheim>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
