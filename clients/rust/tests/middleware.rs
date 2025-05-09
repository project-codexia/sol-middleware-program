#![cfg(feature = "test-sbf")]

use sol_middleware_program::generated::instructions::*;
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

/// Basit bir middleware instruction testi
#[tokio::test]
async fn middleware_instruction_works() {
    let mut context = program_test().start_with_context().await;

    // Test için yeni bir hedef program ve hesap oluştur
    let target_program_id = Pubkey::new_unique();
    let test_account = Keypair::new();

    // MiddlewareBuilder ile instruction oluştur
    let instruction_data = vec![1, 2, 3, 4]; // örnek veri
    let ix = MiddlewareBuilder::new()
        .target_program_id(target_program_id)
        .data(instruction_data.clone())
        .add_account_meta(test_account.pubkey(), true, true)
        .instruction();

    // Transaction oluştur ve gönder
    let payer = &context.payer;
    let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let recent_blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
    transaction.sign(&[payer, &test_account], recent_blockhash);

    // İşlemi gönder ve başarılı olduğunu assert et
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
