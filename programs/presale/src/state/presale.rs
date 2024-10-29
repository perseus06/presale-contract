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
    pub rate: u64,
}