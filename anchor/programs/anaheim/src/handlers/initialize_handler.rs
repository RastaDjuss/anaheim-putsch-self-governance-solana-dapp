use anchor_lang::prelude::*;

use crate::contexts::initialize::Initialize;  // << C’est ça qu’il faut ajouter !

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
  let anaheim = &mut ctx.accounts.anaheim;
  anaheim.authority = ctx.accounts.user.key();
  anaheim.count = 0;
  anaheim.value = 0;
  Ok(())
}
