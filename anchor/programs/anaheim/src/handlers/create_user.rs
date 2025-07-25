// FILE: anchor/programs/anaheim/src/handlers/create_user.rs
use anchor_lang::prelude::*;
use crate::create_user::CreateUser;
use crate::error::ErrorCode;
use crate::constants::MAX_USERNAME_LENGTH;

pub fn handle_create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  if username.len() > MAX_USERNAME_LENGTH {
    return err!(ErrorCode::UsernameTooLong);
  }
  let user_account = &mut ctx.accounts.user_account;

  // CHANGEMENT ici :
  user_account.username = username;
  user_account.authority = *ctx.accounts.authority.key;

  Ok(())
}
