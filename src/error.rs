use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    /// Wrapper for `StdError`
    #[error("{0}")]
    Std(#[from] StdError),

    /// Wrapper for `PaymentError`
    #[error("{0}")]
    Payment(#[from] PaymentError),

    /// Resource not found
    #[error("NotFound")]
    NotFound {},

    /// User is not authorized to access this resource
    #[error("Unauthorized")]
    Unauthorized {},

    /// There is already an active bidding period
    #[error("Bidding Period Active")]
    BiddingPeriodActive {},

    /// The current bidding period has expired
    #[error("Bidding Period Expired")]
    BiddingPeriodExpired {},

    /// Custom error with value
    #[error("Custom Error val: {val:?}")]
    CustomErrorParam { val: String },
}
