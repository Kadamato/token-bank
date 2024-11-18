use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer},
};

use crate::error::UserError;
use crate::state::{Bank, User};

#[derive(Accounts)]
pub struct WithDraw<'info> {
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

impl WithDraw<'_> {
    fn withdraw_valid(&self, start_time: i64) -> bool {
        //   if time diff is less than 30 days then return false
        let clock = Clock::get().unwrap();
        let current_time = clock.unix_timestamp;
        let time_diff = current_time - start_time;
        if time_diff < 30 * 24 * 60 * 60 {
            return false;
        }
        true
    }

    fn caculate_interest(&self, amount: u64) -> u64 {
        amount.saturating_mul(self.bank.interest_rate.saturating_div(100))
    }
}

pub fn process_withdraw(ctx: Context<WithDraw>, amount: u64) -> Result<()> {
    let deposit_amount = ctx.accounts.user.deposit_amount;
    let bank_sol = ctx.accounts.bank_sol.to_account_info();

    let start_time = ctx.accounts.user.start_time;

    // withdraw_valid
    if !ctx.accounts.withdraw_valid(start_time) {
        return Err(UserError::NotEnoughDay.into());
    }

    if amount > deposit_amount {
        return Err(UserError::InsufficientBalance.into());
    }

    let new_interest = ctx.accounts.caculate_interest(amount);
    let new_amount = amount + new_interest;

    let bank = &mut ctx.accounts.bank;
    let user = &mut ctx.accounts.user;

    // Lấy seed và bump của PDA
    let (_pda, bump) = Pubkey::find_program_address(&[b"bank_sol"], ctx.program_id);
    let seeds = &[b"bank_sol".as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    // Tạo CPI context để transfer SOL
    let cpi_accounts = Transfer {
        from: bank_sol.clone(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    // Transfer SOL
    system_program::transfer(cpi_context, new_amount)?;

    // update total balance in bank
    bank.total_balance -= amount;

    // update total deposit in user account
    user.deposit_amount -= amount;

    msg!("You have withdrawn {} tokens", amount);

    Ok(())
}
