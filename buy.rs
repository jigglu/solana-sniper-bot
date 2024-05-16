use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    account::Account, instruction::Instruction, signature::Signer, transaction::Transaction,
};
use spl_token::instruction::approve;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;
use std::str::FromStr;

#[tokio::test]
async fn test() {
    let program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let mint_authority = Pubkey::from_str("9WqfXJ1X3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y").unwrap();
    let user = Pubkey::from_str("9WqfXJ1X3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y").unwrap();
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let token_account = Pubkey::from_str("9WqfXJ1X3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y").unwrap();
    let token_mint = Pubkey::from_str("9WqfXJ1X3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y3Xz3zv3y").unwrap();
    let token_account_info = AccountInfo::new(
        &token_account,
        false,
        true,
        &mut Account::default(),
        &token_program_id,
        false,
        0,
    );
    let token_mint_info = AccountInfo::new(
        &token_mint,
        false,
        false,
        &mut Account::default(),
        &token_program_id,
        false,
        0,
    );
    let user_info = AccountInfo::new(
        &user,
        true,
        false,
        &mut Account::default(),
        &token_program_id,
        false,
        0,
    );
    let mint_authority_info = AccountInfo::new(
        &mint_authority,
        false,
        false,
        &mut Account::default(),
        &token_program_id,
        false,
        0,
    );
    let mut program_test = ProgramTest::new(
        "spl_token",
        program_id,
        processor!(approve),
    );
    program_test.add_account(
        token_account,
        TokenAccount {
            mint: token_mint,
            owner: user,
            amount: 100,
            state: spl_token::state::AccountState::Initialized,
            ..TokenAccount::default()
        },
    );
    program_test.add_account(
        token_mint,
        Mint {
            mint_authority: mint_authority,
            ..Mint::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                token_account_info.clone(),
                user_info.clone(),
                mint_authority_info.clone(),
                token_mint_info.clone(),
            ],
            data: vec![1, 0, 0, 0, 0, 0, 0, 0],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let token_account = banks_client
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();
    let token_account = TokenAccount::unpack(&token_account.data[..]).unwrap();
    assert_eq!(token_account.amount, 100);
}
