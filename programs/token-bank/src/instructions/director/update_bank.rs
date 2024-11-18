use anchor_lang::prelude::*;

use crate::{
    error::{BankError, UserError},
    Bank, User,
};

use super::{DIRECTOR, MAX_INTEREST, MAX_NAME};

#[derive(Accounts)]

pub struct UpdateBank<'info> {
    #[account(
        mut,
        seeds = [b"bank"],
        bump,
        realloc = 8 + Bank::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true
    )]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl UpdateBank<'_> {
    pub fn check_owner(&self) -> bool {
        self.owner.key() == self.bank.owner_address || self.user.role == DIRECTOR
    }

    pub fn check_name_valid(&self, name: String) -> bool {
        if name.len() > MAX_NAME.into() {
            return false;
        }
        true
    }

    pub fn check_interest_rate_valid(&self, interest_rate: u64) -> bool {
        if interest_rate > MAX_INTEREST.into() {
            return false;
        }
        true
    }
}

pub fn process_update_bank(
    ctx: Context<UpdateBank>,
    name: String,
    interest_rate: u64,
) -> Result<()> {
    msg!("check owner {}", ctx.accounts.check_owner());
    msg!(
        "check name valid {}",
        ctx.accounts.check_name_valid(name.clone())
    );
    msg!(
        "check interest rate valid {}",
        ctx.accounts.check_interest_rate_valid(interest_rate)
    );

    if !ctx.accounts.check_owner() {
        return Err(UserError::Unauthorized.into());
    }

    if !ctx.accounts.check_name_valid(name.clone()) {
        return Err(BankError::NameTooLong.into());
    }

    if !ctx.accounts.check_interest_rate_valid(interest_rate) {
        return Err(BankError::InterestRateTooHigh.into());
    }

    //  update info bank
    let bank = &mut ctx.accounts.bank;
    bank.name = name;
    bank.interest_rate = interest_rate;

    msg!("Director updated bank info");

    Ok(())
}
