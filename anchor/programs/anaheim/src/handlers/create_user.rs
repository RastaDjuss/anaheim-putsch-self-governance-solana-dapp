use anchor_lang::prelude::*;
use crate::contexts::create_user::CreateUser;

pub fn handler(_ctx: Context<CreateUser>, _username: String) -> Result<()> {
  // ta logique métier ici
  Ok(())
}
