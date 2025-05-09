use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::processor;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "Welcome to the Codexia Solana Middleware Program: {}",
        program_id
    );
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        return Err(error);
    }
    Ok(())
}
