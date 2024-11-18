use anchor_lang::prelude::*;

use crate::Bank;

#[derive(Accounts)]
pub struct CreateBank<'info> {
    #[account(init, payer = owner, space = 8 + Bank::INIT_SPACE , seeds = [b"bank"],  bump)]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"bank_sol".as_ref()],
        bump,
    )]
    pub bank_sol: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_init_bank<'info>(
    ctx: Context<CreateBank>,
    name: String,
    interest_rate: u64,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    bank.name = name;
    bank.total_balance = 0;
    bank.interest_rate = interest_rate;
    bank.owner_address = *ctx.accounts.owner.key;
    

    Ok(())
}
