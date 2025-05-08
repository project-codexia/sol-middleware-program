use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MetaEntry {
    pub is_signer: bool,
    pub is_writable: bool,
}
