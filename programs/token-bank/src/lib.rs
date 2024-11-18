use anchor_lang::prelude::*;

mod error;
pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

declare_id!("2wNtiJE9ZDnrQGjf616GX4MuPDMsxNWbmQ6C29bFkYC2");

#[program]
pub mod token_bank {
    use super::*;

    pub fn init_user(ctx: Context<CreateUser>) -> Result<()> {
        let _ = process_init_user(ctx);
        Ok(())
    }

    pub fn init_bank(ctx: Context<CreateBank>, name: String, interest: u64) -> Result<()> {
        let _ = process_init_bank(ctx, name, interest);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let _ = process_deposit(ctx, amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<WithDraw>, amount: u64) -> Result<()> {
        let _ = process_withdraw(ctx, amount);
        Ok(())
    }

    pub fn withdraw_all(ctx: Context<WithdrawRoleBased>) -> Result<()> {
        let _ = process_withdraw_role_based(ctx);
        Ok(())
    }

    pub fn update_bank(ctx: Context<UpdateBank>, name: String, interest_rate: u64) -> Result<()> {
        let _ = process_update_bank(ctx, name, interest_rate);
        Ok(())
    }

    pub fn grant_permission(ctx: Context<GrantPermission>, permission: u8) -> Result<()> {
        let _ = process_grant_permission(ctx, permission);
        Ok(())
    }
}
