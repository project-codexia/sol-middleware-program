use borsh::BorshDeserialize;
use middleware_processor::middleware;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::MiddlewareProgramInstruction;

pub mod middleware_processor;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MiddlewareProgramInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MiddlewareProgramInstruction::Middleware(args) => {
            msg!("Instruction: Middleware");
            middleware(accounts, args)
        }
    }
}
