use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
};

use crate::state::dynamic_instruction::DynamicInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct MiddlewareArgs {
    pub(crate) instruction_data: Vec<u8>,
}

pub(crate) fn middleware<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: MiddlewareArgs,
) -> ProgramResult {
    process_middleware_instruction(accounts, args)
}

pub(crate) fn process_middleware_instruction(
    accounts: &[AccountInfo],
    args: MiddlewareArgs,
) -> ProgramResult {
    let mut accounts_iter = accounts.iter();
    let _payer = next_account_info(&mut accounts_iter)?;
    // POW : pay for the subscription

    let dynamic_ix = DynamicInstruction::try_from_slice(&args.instruction_data)?;
    msg!(
        "Invoking target program: {:?}",
        dynamic_ix.target_program_id
    );

    let mut cpi_accounts = Vec::new();
    let mut metas = Vec::new();

    for (_i, meta) in dynamic_ix.account_metas.iter().enumerate() {
        let acc = next_account_info(&mut accounts_iter)?;
        cpi_accounts.push(acc.clone());
        metas.push(AccountMeta {
            pubkey: *acc.key,
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        });
    }

    let ix = Instruction {
        program_id: dynamic_ix.target_program_id,
        accounts: metas,
        data: dynamic_ix.data.clone(),
    };

    invoke(&ix, &cpi_accounts)?;

    Ok(())
}
