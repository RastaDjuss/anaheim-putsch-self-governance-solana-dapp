use anchor_lang::prelude::*;

/// Structure pour représenter un post
#[account]
pub struct Post {
    pub content: String,     // Le contenu du post (max 256 caractères normalement)
    pub author: Pubkey,      // Adresse publique de l'auteur
    pub created_at: i64,     // Timestamp lors de la création
}