use anchor_lang::prelude::*;

#[error_code]
pub enum PresaleError {
    #[msg("Invalid owner.")]
    InvalidOwner,

    #[msg("The current sale type should be private.")]
    PrivateSale, 

    #[msg("The current sale type should be public.")]
    PublicSale, 

    #[msg("The token amount is too much than balance.")]
    TooMuchAmount,

    #[msg("The contract's status should be live.")]
    NotLive,

    #[msg("The amount is not enoguh.")]
    InsufficientBalance,

    #[msg("The token is not matched")]
    DisMatchToken
}
