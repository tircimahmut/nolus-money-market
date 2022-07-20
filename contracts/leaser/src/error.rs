use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Finance(#[from] finance::error::Error),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error(
        "LeaseHealthyLiability% must be less than LeaseMaxLiability% and 
    LeaseInitialLiability% must be less or equal to LeaseHealthyLiability%"
    )]
    IvalidLiability {},

    #[error("ParseError {err:?}")]
    ParseError { err: String },

    #[error("Validation error {msg:?}")]
    ValidationError { msg: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Cannot open lease with zero downpayment")]
    ZeroDownpayment {},
}

pub type ContractResult<T> = core::result::Result<T, ContractError>;
