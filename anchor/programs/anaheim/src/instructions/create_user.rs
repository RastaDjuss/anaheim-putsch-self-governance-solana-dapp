// instructions/handle_create_user

use anchor_lang::prelude::*;

use crate::contexts::create_user::CreateUser;

pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  let user_account = &mut ctx.accounts.user_account;
  user_account.username = username;
  user_account.authority = ctx.accounts.authority.key();
  Ok(())
}

