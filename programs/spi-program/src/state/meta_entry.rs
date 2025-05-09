use borsh::{BorshDeserialize, BorshSerialize};

/// MetaEntry is a struct that represents an entry in the account metas array
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MetaEntry {
    /// The public key of the account
    pub is_signer: bool,
    /// The public key of the account
    pub is_writable: bool,
}
