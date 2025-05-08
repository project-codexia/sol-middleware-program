use borsh::BorshDeserialize;

#[derive(BorshDeserialize, Debug)]
pub struct MetaEntry {
    pub is_signer: bool,
    pub is_writable: bool,
}
