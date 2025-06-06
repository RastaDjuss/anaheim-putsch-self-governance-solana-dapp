#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod state;
pub mod instructions;
pub mod handlers;
pub mod close;
pub mod utils;
pub mod contexts;

use crate::contexts::create_user::CreateUser;
use crate::handlers::create_user;
use crate::contexts::create_user::__client_accounts_create_user;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod anaheim {
  use super::*;
  pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    create_user::handler(ctx, username)
  }
}
