use anchor_lang::prelude::*;
use crate::instructions::Initialize;

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = 0;
  msg!("Anaheim initialized");
  Ok(())
}

pub(crate) fn handle_initialize(_p0: Context<Initialize>) -> Result<()> {
  todo!()
}
