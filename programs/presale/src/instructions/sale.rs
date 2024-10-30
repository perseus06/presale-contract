use anchor_lang::prelude::*;

use crate::{state::*, constants::*, error::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
};
use solana_program::{program::invoke, system_instruction};
use std::mem::size_of;

pub fn token_sale(ctx: Context<SaleManagement>, amount: u64, staked_period:u8) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.presale.status, PresaleError::NotLive);
    require!(accts.presale.token == accts.token_mint.key(), PresaleError::DisMatchToken);

    let sale_type = accts.presale.sale_type;

    if sale_type {
        // send token from token vault account to user's token account
        require!(amount < accts.presale.token_amount, PresaleError::InsufficientBalance);
        accts.presale.token_amount -= amount;
        msg!("token amount {:?}", amount);

        // send token from token vault account to user's token account
        let (_, bump) = Pubkey::find_program_address(&[PRESALE_SEED], ctx.program_id);
        let vault_seeds = &[PRESALE_SEED, &[bump]];
        let signer = &[&vault_seeds[..]];

        // Transfer tokens from bridge to receiver
        let cpi_accounts = Transfer {
            from: accts.token_vault_account.to_account_info(),
            to: accts.token_account.to_account_info(),
            authority: accts.presale.to_account_info(),
        };

        let cpi_context = CpiContext::new(accts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_context.with_signer(signer), amount)?;
        // calculate the sol amount
        let decimal = accts.token_mint.decimals;
        msg!("This token's decimal is {:?}", decimal.clone());

        let token_price = accts.presale.token_price + (accts.presale.rate as u128 * amount as u128 /  10u64.pow(decimal.into()) as u128 / 2) as u64;
        msg!("token_price {:?}", token_price);
        accts.presale.token_price = token_price;

        let sol_amount = (token_price as u128 * amount as u128 / 10u64.pow(decimal.into()) as u128) as u64;
        msg!("sol amount {:?}", sol_amount);


         // Send sol to the vault
         invoke(
            &system_instruction::transfer(
                &accts.user.key(),
                &accts.vault.key(),
                sol_amount
            ),
            &[
                accts.user.to_account_info().clone(),
                accts.vault.clone(),
                accts.system_program.to_account_info().clone(),
            ],
        )?;
        accts.presale.sol_amount += sol_amount;
    } else {
        // Send sol to the vault
        invoke(
            &system_instruction::transfer(
                &accts.user.key(),
                &accts.vault.key(),
                amount
            ),
            &[
                accts.user.to_account_info().clone(),
                accts.vault.clone(),
                accts.system_program.to_account_info().clone(),
            ],
        )?;
        accts.presale.sol_amount += amount;

        let decimal = accts.token_mint.decimals;
        msg!("This token's decimal is {:?}", decimal.clone());
        
        let token_amount = (amount as u128 * 10u64.pow(decimal.into()) as u128 / accts.presale.token_price as u128) as u64;

        require!(token_amount < accts.presale.token_amount, PresaleError::InsufficientBalance);
        accts.presale.token_amount -= token_amount;

        let current_timestamp = Clock::get()?.unix_timestamp;

        match staked_period {
            3_u8 => {
                require!(!accts.user_info.stake_status_3m, PresaleError::AlreadyStaking);
                accts.user_info.stake_status_3m = true;
                accts.user_info.stake_amount_3m = token_amount;
                accts.user_info.stake_start_time_3m = current_timestamp;
            },
            6_u8 => {
                // Logic for 6-month staking period
                require!(!accts.user_info.stake_status_6m, PresaleError::AlreadyStaking);
                accts.user_info.stake_status_6m = true;
                accts.user_info.stake_amount_6m = token_amount;
                accts.user_info.stake_start_time_6m = current_timestamp;
            }, 
            9_u8 => {
                // Logic for 9-month staking period
                require!(!accts.user_info.stake_status_9m, PresaleError::AlreadyStaking);
                accts.user_info.stake_status_9m = true;
                accts.user_info.stake_amount_9m = token_amount;
                accts.user_info.stake_start_time_9m = current_timestamp;
            },
            12_u8 => {
                // Logic for 12-month staking period
                require!(!accts.user_info.stake_status_12m, PresaleError::AlreadyStaking);
                accts.user_info.stake_status_12m = true;
                accts.user_info.stake_amount_12m = token_amount;
                accts.user_info.stake_start_time_12m = current_timestamp;
            },
            _ => return Err(PresaleError::InvalidStakingPeriod.into()), // Handle unsupported values
        }
    }

    Ok(())
}


pub fn claim_staked_token(ctx: Context<SaleManagement>, staked_period:u8) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.presale.status, PresaleError::NotLive);
    let current_timestamp = Clock::get()?.unix_timestamp;
    let mut token_amount = 0;

    match staked_period {
        3_u8 => {
            require!(accts.user_info.stake_status_3m, PresaleError::NotStaking);
            require!(current_timestamp  - accts.user_info.stake_start_time_3m >  3 * 30 * 3600 * 24, PresaleError::NotStaking);
            require!(accts.user_info.stake_amount_3m != 0, PresaleError::AlreadyClaim);

            token_amount = accts.user_info.stake_amount_3m * 105 / 100;
            accts.user_info.stake_amount_3m = 0;
            accts.user_info.stake_start_time_3m = current_timestamp;
        },
        6_u8 => {
            // Logic for 6-month staking period
            require!(accts.user_info.stake_status_6m, PresaleError::NotStaking);
            require!(current_timestamp  - accts.user_info.stake_start_time_3m > 6 * 30 * 3600 * 24, PresaleError::NotStaking);
            require!(accts.user_info.stake_amount_6m != 0, PresaleError::AlreadyClaim);

            token_amount = accts.user_info.stake_amount_3m * 110 / 100;
            accts.user_info.stake_amount_6m = 0;
            accts.user_info.stake_start_time_6m = current_timestamp;
        }, 
        9_u8 => {
            // Logic for 9-month staking period
            require!(accts.user_info.stake_status_9m, PresaleError::NotStaking);
            require!(current_timestamp  - accts.user_info.stake_start_time_3m > 9 * 30 * 3600 * 24, PresaleError::NotStaking);
            require!(accts.user_info.stake_amount_9m != 0, PresaleError::AlreadyClaim);

            token_amount = accts.user_info.stake_amount_3m * 115 / 100;
            accts.user_info.stake_amount_9m = 0;
            accts.user_info.stake_start_time_9m = current_timestamp;
        },
        12_u8 => {
            // Logic for 12-month staking period
            require!(accts.user_info.stake_status_12m, PresaleError::NotStaking);
            require!(current_timestamp  - accts.user_info.stake_start_time_3m > 12 * 30 * 3600 * 24, PresaleError::NotStaking);
            require!(accts.user_info.stake_amount_12m != 0, PresaleError::AlreadyClaim);

            token_amount = accts.user_info.stake_amount_3m * 120 / 100;
            accts.user_info.stake_amount_12m = 0;
            accts.user_info.stake_start_time_12m = current_timestamp;
        },
        _ => return Err(PresaleError::InvalidStakingPeriod.into()), // Handle unsupported values
    }

    // send token from token vault account to user's token account
    let (_, bump) = Pubkey::find_program_address(&[PRESALE_SEED], ctx.program_id);
    let vault_seeds = &[PRESALE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    // Transfer tokens from bridge to receiver
    let cpi_accounts = Transfer {
        from: accts.token_vault_account.to_account_info(),
        to: accts.token_account.to_account_info(),
        authority: accts.presale.to_account_info(),
    };

    let cpi_context = CpiContext::new(accts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context.with_signer(signer), token_amount)?;

    Ok(())
}
#[derive(Accounts)]
pub struct SaleManagement<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump,
        space = 8 + size_of::<UserInfo>()
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut, 
        seeds = [PRESALE_SEED],
        bump
    )]
    pub presale: Box<Account<'info, Presale>>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub token_account: Box<Account<'info, TokenAccount>>, // the token account of owner

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = presale
    )]
    pub token_vault_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

