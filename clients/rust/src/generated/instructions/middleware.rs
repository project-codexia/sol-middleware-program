#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};

/// Bir account için meta bilgi
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaEntry {
    /// Hesabın public key'i
    pub pubkey: solana_program::pubkey::Pubkey,
    /// Hesap imzalayıcı mı
    pub is_signer: bool,
    /// Hesap yazılabilir mi
    pub is_writable: bool,
}

/// Dinamik olarak başka bir programa instruction göndermek için yapı
pub struct DynamicInstruction {
    /// Hedef program id
    pub target_program_id: solana_program::pubkey::Pubkey,
    /// Instruction datası
    pub data: Vec<u8>,
    /// Hedef programa iletilecek hesaplar
    pub account_metas: Vec<MetaEntry>,
}

impl DynamicInstruction {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }

    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(self.account_metas.len() + remaining_accounts.len());

        for meta in &self.account_metas {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: meta.pubkey,
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            });
        }

        accounts.extend_from_slice(remaining_accounts);

        solana_program::instruction::Instruction {
            program_id: self.target_program_id,
            accounts,
            data: self.data.clone(),
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MiddlewareArgs {
    /// Hedef programa gönderilecek instruction datası
    pub instruction_data: Vec<u8>,
}

#[derive(Default)]
pub struct MiddlewareBuilder {
    target_program_id: Option<solana_program::pubkey::Pubkey>,
    data: Option<Vec<u8>>,
    account_metas: Vec<MetaEntry>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MiddlewareBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Hedef program id'sini ayarla
    pub fn target_program_id(mut self, target_program_id: solana_program::pubkey::Pubkey) -> Self {
        self.target_program_id = Some(target_program_id);
        self
    }

    /// Instruction datasını ayarla
    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Account ekle
    pub fn add_account_meta(
        mut self,
        pubkey: solana_program::pubkey::Pubkey,
        is_signer: bool,
        is_writable: bool,
    ) -> Self {
        self.account_metas.push(MetaEntry {
            pubkey,
            is_signer,
            is_writable,
        });
        self
    }

    /// Ekstra account ekle (opsiyonel)
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }

    /// Birden fazla ekstra account ekle (opsiyonel)
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }

    /// Instruction oluştur
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let dynamic = DynamicInstruction {
            target_program_id: self
                .target_program_id
                .expect("target_program_id is required"),
            data: self.data.clone().expect("data is required"),
            account_metas: self.account_metas.clone(),
        };

        dynamic.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}
// Usage example:
// let ix = MiddlewareBuilder::new()
//     .target_program_id(target_program_id)
//     .data(instruction_data)
//     .add_account_meta(account_pubkey, true, true)
//     .add_account_meta(another_pubkey, false, false)
//     .instruction();
