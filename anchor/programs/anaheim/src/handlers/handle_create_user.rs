use anchor_lang::prelude::*;
use crate::contexts::create_user::CreateUser;
use crate::handlers::create_user::handler;

pub fn handle_create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
  handler(ctx, username)
}
