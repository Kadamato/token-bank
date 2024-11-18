use anchor_lang::prelude::*;

#[error_code]
pub enum UserError {
    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Unauthorized bank")]
    UnauthorizedBank,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Not enough day to withdraw")]
    NotEnoughDay,

    #[msg("Invalid permission")]
    InvalidPermission,
}
