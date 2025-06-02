use anchor_lang::prelude::*;
use crate::contexts::Update;

pub fn handle_increment(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.value() += 1;
  Ok(())
}
