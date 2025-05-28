use anchor_lang::prelude::*;
use crate::state::PostAccount;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
  #[account(mut, close = destination)] // Ferme un compte et envoie les restes au destinataire
  pub account_to_close: Account<'info, PostAccount>, // Exemple si PostAccount est attendu

  #[account(mut)] // Le destinataire doit pouvoir recevoir ce solde
  pub destination: Signer<'info>,
}

pub fn handler(ctx: Context<CloseAccount>) -> Result<()> {
  let account_to_close = &mut ctx.accounts.account_to_close;
  let destination = ctx.accounts.destination.to_account_info();

  // Fermer le compte et transférer son solde à destination
  // Implémentez ici la logique nécessaire selon votre cas d'utilisation

  Ok(())
}
