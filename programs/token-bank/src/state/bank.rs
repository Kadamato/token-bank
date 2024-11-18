use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub owner_address: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub total_balance: u64,
    pub interest_rate: u64,
}
