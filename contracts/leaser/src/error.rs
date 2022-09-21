use std::any::type_name;

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("[Leaser] [Std] {0}")]
    Std(#[from] StdError),

    #[error("[Leaser] {0}")]
    Finance(#[from] finance::error::Error),

    #[error("[Leaser] {0}")]
    Lpp(#[from] lpp::error::ContractError),

    #[error("[Leaser] {0}")]
    Platform(#[from] platform::error::Error),

    #[error("[Leaser] Unauthorized")]
    Unauthorized {},

    #[error(
        "[Leaser] LeaseHealthyLiability% must be less than LeaseMaxLiability% and LeaseInitialLiability% must be less or equal to LeaseHealthyLiability%"
    )]
    IvalidLiability {},

    #[error("[Leaser] ParseError {err:?}")]
    ParseError { err: String },

    #[error("[Leaser] Validation {0}")]
    Validation(String),

    #[error("[Leaser] Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("[Leaser] Cannot open lease with zero downpayment")]
    ZeroDownpayment {},

    #[error("[Leaser] Unknown currency symbol: {symbol:?}")]
    UnknownCurrency { symbol: String },

    #[error("[Leaser] No Liquidity")]
    NoLiquidity {},
}

impl ContractError {
    pub fn validation_err<T>(str: String) -> Self {
        Self::Validation(format!("[ {} ] {}", String::from(type_name::<T>()), str))
    }
}

pub type ContractResult<T> = core::result::Result<T, ContractError>;
