use anchor_lang::prelude::*;
use crate::contexts::Update;

pub fn decrement(ctx: Context<Update>) -> Result<()> {
  handle_decrement(ctx)
}

fn handle_decrement(_ctx: Context<Update>) -> Result<()> {
  // implémentation à venir
  Ok(())
}
