use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::ProgramInstruction;

pub mod middleware_processor;
pub use middleware_processor::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    match instruction {
        ProgramInstruction::MiddlewareInstruction { instruction_data } => {
            msg!("Processing MiddlewareInstruction");
            process_middleware_instruction(program_id, accounts, &instruction_data)
        }
    }
}
