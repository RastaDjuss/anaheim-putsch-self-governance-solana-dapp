// FILE: anchor/programs/anaheim/src/handlers/initialize_handler.rs
use anchor_lang::prelude::*;
use super::super::instructions::initialize::Initialize;

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let account = &mut ctx.accounts.anaheim;
    account.authority = *ctx.accounts.payer.key;
    account.count = 0;
    account.value = 0;
    Ok(())
}