pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("BTNaNtGC5sTfNUbusLBRuMViPT2wNBvkzCem5HEDBUMM");

#[program]
pub mod presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, token_price: u64) -> Result<()> {
        instructions::initialize(ctx, amount, token_price)
    }

    pub fn toggle_status(ctx: Context<ManagePresale>) -> Result<()> {
        instructions::toggle_status(ctx)
    }

    pub fn update_sale_type(ctx: Context<ManagePresale>) -> Result<()> {
        instructions::update_sale_type(ctx)
    }

    pub fn update_token_price(ctx: Context<ManagePresale>, new_price: u64) -> Result<()> {
        instructions::update_token_price(ctx, new_price)
    }

    pub fn update_owner(ctx: Context<ManagePresale>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }

    pub fn update_rate(ctx: Context<ManagePresale>, rate: u64) -> Result<()> {
        instructions::update_rate(ctx, rate)
    }

    pub fn token_sale(ctx: Context<SaleManagement>, amount: u64, staked_period: u8) -> Result<()> {
        instructions::token_sale(ctx, amount, staked_period)
    }

    pub fn claim_staked_token(ctx: Context<SaleManagement>, staked_period: u8) -> Result<()> {
        instructions::claim_staked_token(ctx, staked_period)
    }

    pub fn deposit_token(ctx: Context<ManageToken>, amount: u64) -> Result<()> {
        instructions::deposit_token(ctx, amount)
    }

    pub fn withdraw_token(ctx: Context<ManageToken>, amount: u64) -> Result<()> {
        instructions::withdraw_token(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
