use anchor_lang::prelude::*;
use crate::contexts::update::Update;

pub fn handle_decrement(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.value = anaheim.value.saturating_sub(1);
  Ok(())
}
