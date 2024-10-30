use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Presale {
    pub owner: Pubkey, // Contract owner
    pub vault: Pubkey, // Sol vault address of the contract
    pub token_vault: Pubkey, // Presale token address of the contract - escrow vault
    pub token: Pubkey, // Presale token address
    pub token_amount: u64, // Token balance of the contract
    pub sol_amount: u64, // Sol amount of the value on contract
    pub token_price: u64, // the token price will be expressed with sol
    pub status: bool, // contract's status
    pub sale_type: bool, // false: Private Sale, true: Public Sale
    pub rate: u64, // token price increase rate
    pub staked_amount: u64, // total token amount in staking
    pub rate_3m: u64, // the yield rate for 3 months period
    pub rate_6m: u64, // the yield rate for 3 months period
    pub rate_9m: u64, // the yield rate for 3 months period
    pub rate_12m: u64, // the yield rate for 3 months period
}

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub user: Pubkey,

    pub stake_amount_3m: u64,       // Stake amount for 3 months
    pub stake_start_time_3m: i64,   // Start time for 3-month stake
    pub stake_status_3m: bool,   // Status for 3-month stake

    pub stake_amount_6m: u64,       // Stake amount for 6 months
    pub stake_start_time_6m: i64,   // Start time for 6-month stake
    pub stake_status_6m: bool,   // Status for 6-month stake

    pub stake_amount_9m: u64,       // Stake amount for 9 months
    pub stake_start_time_9m: i64,   // Start time for 9-month stake
    pub stake_status_9m: bool,   // Status for 9-month stake

    pub stake_amount_12m: u64,      // Stake amount for 12 months
    pub stake_start_time_12m: i64,  // Start time for 12-month stake
    pub stake_status_12m: bool,   // Status for 12-month stake
}
