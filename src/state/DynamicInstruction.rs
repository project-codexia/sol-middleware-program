use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;

use super::MetaEntry::*;

#[derive(BorshDeserialize, Debug)]
pub struct DynamicInstruction {
    pub target_program_id: Pubkey,
    pub data: Vec<u8>,
    pub account_metas: Vec<MetaEntry>,
}
