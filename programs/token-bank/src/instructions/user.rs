use anchor_lang::prelude::*;

use crate::User;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = owner, space = 8 + User::INIT_SPACE, seeds = [b"user", owner.key().as_ref()], bump)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_init_user(ctx: Context<CreateUser>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.role = 0;
    user.withdraw_total = 0;
    user.deposit_amount_total = 0;
    user.deposit_amount = 0;
    user.start_time = 0;
    user.end_time = 0;
    Ok(())
}
