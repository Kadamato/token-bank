use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub role: u8, // 0: user, 1: director, 2: sales
    pub withdraw_total: u64,
    pub deposit_amount_total: u64,
    pub deposit_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
}
