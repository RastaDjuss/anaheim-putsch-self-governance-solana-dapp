use anchor_lang::prelude::*;
use crate::contexts::initialize::Initialize;

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
  msg!("Anaheim initialized");

  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.authority = ctx.accounts.user.key();
  anaheim.count = 0;
  anaheim.value = 0;

  Ok(())
}
