use anchor_lang::prelude::*;

#[error_code]
pub enum PresaleError {
    #[msg("Invalid owner.")]
    InvalidOwner,

    #[msg("The current sale type should be private.")]
    PrivateSale, 

    #[msg("The current sale type should be public.")]
    PublicSale, 

    #[msg("The contract's status should be live.")]
    NotLive,

    #[msg("The amount is not enoguh.")]
    InsufficientBalance,

    #[msg("The token is not matched.")]
    DisMatchToken,

    #[msg("You already staked token in this period.")]
    AlreadyStaking,

    #[msg("The staking period is invalid.")]
    InvalidStakingPeriod,

    #[msg("You didn't stake in this period.")]
    NotStaking, 

    #[msg("You already claimed token in this period.")]
    AlreadyClaim
}
