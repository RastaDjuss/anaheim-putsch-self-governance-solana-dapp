use anchor_lang::prelude::*;
use crate::contexts::update::Update;

pub fn handler(ctx: Context<Update>, value: u8) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.count = value as u64;
  Ok(())
}
