use anchor_lang::prelude::*;

use crate::{state::*, constants::*, error::*};

use std::mem::size_of;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
};

pub fn initialize(
    ctx: Context<Initialize>,
    amount: u64,
    token_price: u64
) -> Result<()> {
    let accts = ctx.accounts;

    // update the presale account with data
    accts.presale.owner = accts.owner.key();
    accts.presale.vault = accts.vault.key();
    accts.presale.token_vault = accts.token_vault_account.key();
    accts.presale.token = accts.token_mint.key();
    accts.presale.balance = amount;
    accts.presale.token_price = token_price;
    accts.presale.status = false;
    accts.presale.sale_type = false;

    // send presale token to the contract
    let cpi_accounts = Transfer {
        from: accts.token_account.to_account_info(),
        to: accts.token_vault_account.to_account_info(),
        authority: accts.owner.to_account_info(),
    };
    let cpi_context = CpiContext::new(accts.token_program.to_account_info(), cpi_accounts);
    let _ = token::transfer(cpi_context, amount);

    Ok(())
}

pub fn toggle_status(ctx: Context<ManagePresale>) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.owner.key() == accts.presale.owner, PresaleError::InvalidOwner);
    // toggle contract's status
    accts.presale.status = !accts.presale.status;

    Ok(())
}

pub fn update_sale_type(ctx: Context<ManagePresale>) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.owner.key() == accts.presale.owner, PresaleError::InvalidOwner);
    require!(accts.presale.sale_type == false, PresaleError::PrivateSale);
    // update sale type
    accts.presale.sale_type = true;

    Ok(())
}

pub fn update_token_price(ctx: Context<ManagePresale>, new_price: u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.owner.key() == accts.presale.owner, PresaleError::InvalidOwner);
    require!(accts.presale.sale_type == false, PresaleError::PrivateSale);

    // update token price and it is avaiable in private sale
    accts.presale.token_price = new_price;

    Ok(())
}

pub fn update_owner(ctx: Context<ManagePresale>, new_owner: Pubkey) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.owner.key() == accts.presale.owner, PresaleError::InvalidOwner);

    // update token price and it is avaiable in private sale
    accts.presale.owner = new_owner;

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init, 
        payer = owner, 
        seeds = [PRESALE_SEED],
        bump,
        space = 8 + size_of::<Presale>()
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
        init,
        payer = owner,
        seeds = [TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = presale
    )]
    pub token_vault_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ManagePresale<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut, 
        seeds = [PRESALE_SEED],
        bump,
    )]
    pub presale: Box<Account<'info, Presale>>,
}