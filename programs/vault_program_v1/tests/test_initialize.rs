use {
    anchor_lang::{
        solana_program::instruction::Instruction, system_program::ID as SYSTEM_PROGRAM_ID,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::Message,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

fn setup() -> (LiteSVM, Keypair) {
    let program_id = vault_program_v1::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault_program_v1.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    (svm, payer)
}

#[test]
fn test_initialize_deposit_withdraw_close() {
    let (mut svm, payer) = setup();
    let user = payer.pubkey();

    let (vault_state_pda, state_bump) =
        Pubkey::find_program_address(&[b"state", user.as_ref()], &vault_program_v1::id());

    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[b"vault", vault_state_pda.as_ref()],
        &vault_program_v1::id(),
    );

    // initialize
    let init_ix = Instruction {
        program_id: vault_program_v1::id(),
        accounts: vault_program_v1::accounts::Initialize {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: vault_program_v1::instruction::Initialize {}.data(),
    };

    let message = Message::new(&[init_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    let tx1 = svm.send_transaction(transaction).unwrap();

    println!("Initialize transaction successful");
    println!("Tx Signature: {}", tx1.signature);

    let vault_state_account = svm.get_account(&vault_state_pda).unwrap();
    let vault_state = vault_program_v1::state::VaultState::try_deserialize(
        &mut vault_state_account.data.as_ref(),
    )
    .unwrap();

    assert_eq!(vault_state.vault_bump, vault_bump);
    assert_eq!(vault_state.state_bump, state_bump);

    // Deposit 1 Sol
    let deposit_amount = 1_000_000_000;

    let deposit_ix = Instruction {
        program_id: vault_program_v1::id(),
        accounts: vault_program_v1::accounts::Deposit {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: vault_program_v1::instruction::Deposit {
            amount: deposit_amount,
        }
        .data(),
    };

    let message = Message::new(&[deposit_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction2 = Transaction::new(&[&payer], message, recent_blockhash);
    let tx2 = svm.send_transaction(transaction2).unwrap();

    println!("Deposit transaction successful");
    println!("Tx Signature: {}", tx2.signature);

    let vault_balance_after_deposit = svm.get_balance(&vault_pda).unwrap();
    assert_eq!(vault_balance_after_deposit, deposit_amount);
    println!("Balance after deposit {} ", vault_balance_after_deposit);

    //Withdraw
    let withdraw_amount = 500_000_000;

    let withdraw_ix = Instruction {
        program_id: vault_program_v1::id(),
        accounts: vault_program_v1::accounts::Withdraw {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: vault_program_v1::instruction::Withdraw {
            amount: withdraw_amount,
        }
        .data(),
    };

    let message = Message::new(&[withdraw_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction3 = Transaction::new(&[&payer], message, recent_blockhash);
    let tx3 = svm.send_transaction(transaction3).unwrap();

    println!("Withdraw transaction successful");
    println!("Tx Signature: {}", tx3.signature);

    let vault_balance_after_withdraw = svm.get_balance(&vault_pda).unwrap();
    assert_eq!(vault_balance_after_withdraw, withdraw_amount);
    println!("Balance after withdraw {} ", vault_balance_after_withdraw);

    //Close
    let close_amount = svm.get_balance(&vault_pda).unwrap();

    let close_ix = Instruction {
        program_id: vault_program_v1::id(),
        accounts: vault_program_v1::accounts::Close {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: vault_program_v1::instruction::Close {}.data(),
    };

    let message = Message::new(&[close_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction4 = Transaction::new(&[&payer], message, recent_blockhash);
    let tx4 = svm.send_transaction(transaction4).unwrap();

    println!("Close transaction successful");
    println!("Tx Signature: {}", tx4.signature);

    let user_balance_after_close = svm.get_balance(&user).unwrap();
    assert!(user_balance_after_close > close_amount);
    println!("Close {} ", user_balance_after_close);
}
