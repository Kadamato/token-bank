use anchor_lang::{prelude::*, system_program};

use anchor_lang::system_program::Transfer;

use crate::error::UserError;
use crate::state::{Bank, User};

use crate::director::config::USER;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub bank_sol: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn check_user_valid(&self) -> bool {
        self.user.role == USER
    }
}
pub fn process_deposit<'info>(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if !ctx.accounts.check_user_valid() {
        return Err(UserError::Unauthorized.into());
    }

    let clock: Clock = Clock::get()?;

    let user = &mut ctx.accounts.user;

    //  check amount in owner account
    let owner_balance = ctx.accounts.owner.to_account_info().lamports();
    if owner_balance < amount {
        return Err(UserError::InsufficientBalance.into());
    }

    //     transfer amount from owner to bank
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.bank_sol.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    system_program::transfer(cpi_context, amount)?;

    //      update total balance in bank
    let bank = &mut ctx.accounts.bank;
    bank.total_balance += amount;

    //     update total deposit in user account
    user.deposit_amount_total += amount;
    user.deposit_amount = amount;

    user.role = USER;

    user.start_time = clock.unix_timestamp;

    msg!("Deposited {} tokens", amount);
    Ok(())
}
