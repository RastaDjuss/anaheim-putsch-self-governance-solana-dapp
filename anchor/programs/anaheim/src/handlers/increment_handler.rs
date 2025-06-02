use anchor_lang::prelude::*;
use crate::contexts::Update;

pub fn handle_increment(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.value = anaheim.value.saturating_add(1);
  Ok(())
}
