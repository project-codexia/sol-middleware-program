use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use super::meta_entry::*;

/// DynamicInstruction is a struct that represents a dynamic instruction
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct DynamicInstruction {
    /// The program ID of the target program
    pub target_program_id: Pubkey,
    /// The instruction data to be passed to the target program
    pub data: Vec<u8>,
    /// The accounts to be passed to the target program
    pub account_metas: Vec<MetaEntry>,
}
