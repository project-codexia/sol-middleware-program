#![allow(missing_docs)]
use crate::processor::MiddlewareArgs;
use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

/// Instructions supported by the spi program.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction, PartialEq)]
#[rustfmt::skip]
pub(crate) enum MiddlewareProgramInstruction {
    /// Create a new middleware
    #[account(
        0,
        optional,
        name = "_payer",
        desc = "payer who pays for the subscription"
    )]
    Middleware(MiddlewareArgs),
}
