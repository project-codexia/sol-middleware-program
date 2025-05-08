use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::processor::middleware_processor::MiddlewareArgs;

#[derive(
    BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction, PartialEq,
)]
pub(crate) enum MiddlewareProgramInstruction {
    #[account(
        0,
        optional,
        name = "payer",
        desc = "payer who pays for the subscription"
    )]
    Middleware(MiddlewareArgs),
}
