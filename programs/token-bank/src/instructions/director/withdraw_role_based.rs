use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{self, Transfer},
};

use crate::{Bank, User};

use crate::director::config::{DIRECTOR, SALE};

#[derive(Accounts)]

pub struct WithdrawRoleBased<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub bank_sol: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl WithdrawRoleBased<'_> {
    pub fn check_owner(&self) -> bool {
        self.bank.owner_address == self.owner.key() || self.user.role == DIRECTOR
    }

    pub fn check_sale(&self) -> bool {
        self.user.role == SALE
    }

    pub fn verify_role(&self) -> Result<()> {
        if !self.check_owner() && !self.check_sale() {
            return Err(WithdrawAllError::Unauthorized.into());
        }
        Ok(())
    }

    pub fn transfer(&self, ctx: &Context<WithdrawRoleBased>, amount: u64) -> Result<()> {
        // Lấy seed và bump của PDA
        let (_pda, bump) = Pubkey::find_program_address(&[b"bank_sol"], ctx.program_id);
        let seeds = &[b"bank_sol".as_ref(), &[bump]];
        let signer_seeds = &[&seeds[..]];

        // Tạo CPI context để transfer SOL
        let cpi_accounts = Transfer {
            from: self.bank_sol.to_account_info(),
            to: self.owner.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        // Transfer SOL
        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }
}

pub fn process_withdraw_role_based<'info>(ctx: Context<WithdrawRoleBased<'info>>) -> Result<()> {
    ctx.accounts.verify_role()?;

    let director = ctx.accounts.check_owner();
    let sale: bool = ctx.accounts.check_sale();

    let bank_total_balance = ctx.accounts.bank.total_balance;

    let mut amount: u64 = 1;

    if director {
        //  withdraw all
        amount = bank_total_balance;
        msg!("Director withdrawn {} tokens", amount / LAMPORTS_PER_SOL);
    }
    if sale {
        //  withdraw max 10%
        amount = bank_total_balance.saturating_div(10);
        msg!("Sale withdrawn {} tokens", amount / LAMPORTS_PER_SOL);
    }

    ctx.accounts.transfer(&ctx, amount)?;

    let user = &mut ctx.accounts.user;
    let bank = &mut ctx.accounts.bank;

    bank.total_balance -= amount;
    user.withdraw_total += amount;

    Ok(())
}

#[error_code]
pub enum WithdrawAllError {
    #[msg("Unauthorized")]
    Unauthorized,
}
