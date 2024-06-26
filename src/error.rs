use cosmwasm_std::{DivideByZeroError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Divide by zero")]
    DivideByZeroError(#[from] DivideByZeroError),
    #[error("No claims that can be released currently")]
    NothingToClaim {},
    #[error("Must send '{0}' to stake")]
    MissingDenom(String),
    #[error("Sent unsupported denoms, must send '{0}' to stake")]
    ExtraDenoms(String),
    #[error("Must send valid address to stake")]
    InvalidDenom(String),
    #[error("Missed address or denom")]
    MixedNativeAndCw20(String),
    #[error("No funds sent")]
    NoFunds {},
    #[error("No data in ReceiveMsg")]
    NoData {},
}
