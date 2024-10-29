use anchor_lang::prelude::*;

use crate::{state::*, constants::*, error::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
};
use solana_program::{program::invoke, system_instruction};

pub fn token_sale(ctx: Context<SaleManagement>, sol_amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.presale.status, PresaleError::NotLive);

    let sale_type = accts.presale.sale_type;

    invoke(
        &system_instruction::transfer(
            &accts.user.key(),
            &accts.vault.key(),
            sol_amount.clone()
        ),
        &[
            accts.user.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;

    if sale_type {

    } else {
        // Send sol to the vault
        let decimal = accts.token_mint.decimals;
        msg!("This token's decimal is {:?}", decimal.clone());
        
        let amount = (sol_amount as u128 * 10u64.pow(decimal.into()) as u128 / accts.presale.token_price as u128) as u64;

        require!(amount.clone() < accts.presale.balance, PresaleError::TooMuchAmount);
        
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
    }

    Ok(())
}

#[derive(Accounts)]
pub struct SaleManagement<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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

    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>, // the token account of owner

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = presale
    )]
    pub token_vault_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}