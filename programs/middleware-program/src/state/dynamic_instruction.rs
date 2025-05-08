use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use super::meta_entry::*;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct DynamicInstruction {
    pub target_program_id: Pubkey,
    pub data: Vec<u8>,
    pub account_metas: Vec<MetaEntry>,
}
