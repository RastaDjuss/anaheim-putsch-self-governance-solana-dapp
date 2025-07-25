// FILE: anchor/programs/anaheim/src/handlers/initialize_handler.rs
use crate::contexts::initialize::Initialize; // ← ou le chemin réel
use anchor_lang::prelude::*;

pub fn initialize_handler(_ctx: Context<Initialize>) -> Result<()> {
    // logique ici
    Ok(())
}
