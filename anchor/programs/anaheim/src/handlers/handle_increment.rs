use anchor_lang::prelude::*;
use crate::contexts::update::Update;

pub fn handle_increment(ctx: Context<Update>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count += 1;
  Ok(())
}

