use anchor_lang::prelude::*;

use crate::Bank;
use crate::{error::UserError, User};

use crate::config::{DIRECTOR, SALE};

use super::USER;

#[derive(Accounts)]
pub struct GrantPermission<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl GrantPermission<'_> {
    pub fn check_owner(&self) -> bool {
        self.bank.owner_address == self.owner.key() || self.user.role == DIRECTOR
    }

    pub fn check_permission_valid(&self, permission: u8) -> bool {
        if permission != DIRECTOR && permission != SALE && permission != USER {
            return false;
        }
        true
    }
}

pub fn process_grant_permission(ctx: Context<GrantPermission>, permission: u8) -> Result<()> {
    if !ctx.accounts.check_owner() {
        return Err(UserError::Unauthorized.into());
    }

    if !ctx.accounts.check_permission_valid(permission) {
        return Err(UserError::InvalidPermission.into());
    }

    let user = &mut ctx.accounts.user;
    user.role = permission;

    msg!("Director granted permission to user");

    Ok(())
}
