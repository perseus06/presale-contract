use anchor_lang::prelude::*;

#[error_code]
pub enum PresaleError {
    #[msg("Invalid owner.")]
    InvalidOwner,
    
    #[msg("The current sale type should be private.")]
    PrivateSale
}
