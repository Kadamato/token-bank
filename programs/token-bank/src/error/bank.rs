use anchor_lang::prelude::*;

#[error_code]

pub enum BankError {
    #[msg("The provided name is too long.")]
    NameTooLong,

    #[msg("The interest rate is too high.")]
    InterestRateTooHigh,
}
