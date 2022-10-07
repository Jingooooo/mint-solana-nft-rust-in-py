use mpl_token_metadata::{instruction::*, pda::find_metadata_account, state::*};
use pyo3::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, program_pack::Pack, pubkey::Pubkey, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use spl_token::state::Mint;

// fn get_keypair_from_path(path: &str) -> Keypair {
//     let string_keypair = fs::read_to_string(path).unwrap();
//     let raw_keypair: &[u8] = serde_json::from_str(&string_keypair).unwrap();

//     let keypair: Keypair = Keypair::from_bytes(string_keypair.as_bytes()).unwrap();

//     return keypair;
// }

#[pyfunction]
fn send_transaction() {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let secret_key: [u8; 64] = [
        // add secret_key
    ];

    let wallet = Keypair::from_bytes(&secret_key).unwrap();
    let mint_account = Keypair::new();
    let mint_authority_account = &wallet;
    let metadata_account = find_metadata_account(&mint_account.pubkey()).0;

    let name = "testest";
    let symbol = "TTT";
    let uri = "www.naver.com";
    let seller_fee_basis_point = 100;
    let creators = vec![Creator {
        address: wallet.pubkey(),
        verified: false,
        share: 100,
    }];

    let decimals = 0;

    let space = Mint::LEN;

    let minimum_balance_for_rent_exemption = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    let assoc = spl_associated_token_account::get_associated_token_address(
        &wallet.pubkey(),
        &mint_account.pubkey(),
    );

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &wallet.pubkey(),
        &mint_account.pubkey(),
        minimum_balance_for_rent_exemption,
        space as u64,
        &spl_token::ID,
    );

    let initialize_mint_instruction: Instruction = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint_account.pubkey(),
        &wallet.pubkey(),
        None,
        decimals,
    )
    .unwrap();

    let create_assoc_instruction = spl_associated_token_account::create_associated_token_account(
        &wallet.pubkey(),
        &wallet.pubkey(),
        &mint_account.pubkey(),
    );

    let mint_token_to = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint_account.pubkey(),
        &assoc,
        &wallet.pubkey(),
        &[&wallet.pubkey()],
        1,
    )
    .unwrap();

    let create_metadata_account = mpl_token_metadata::instruction::create_metadata_accounts_v2(
        mpl_token_metadata::ID,
        metadata_account,
        mint_account.pubkey(),
        mint_authority_account.pubkey(),
        wallet.pubkey(),
        wallet.pubkey(),
        name.to_string(),
        symbol.to_string(),
        uri.to_string(),
        Some(creators),
        seller_fee_basis_point,
        false,
        true,
        None,
        None,
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[
            create_account_instruction,
            initialize_mint_instruction,
            create_assoc_instruction,
            mint_token_to,
            create_metadata_account,
        ],
        Some(&wallet.pubkey()),
        &[&wallet, &mint_account],
        recent_blockhash,
    );
    let result = client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .unwrap();

    println!("Signature : {}", result.to_string());
    println!("NFT Tokens minted successfully.");
}

// #[pyfunction]
// fn send_transaction(transaction: Transaction) -> PyResult<()> {
//     let client = RpcClient::new("https://api.devnet.solana.com");
//
//     let result = client
//         .send_and_confirm_transaction_with_spinner(&transaction)
//         .unwrap();
//
//     println!("Signature : {}", result.to_string());
//     println!("NFT Tokens minted successfully.");
//     Ok(())
// }

#[pymodule]
fn rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_transaction, m)?)?;
//     m.add_function(wrap_pyfunction!(send_transaction, m)?)?;

    Ok(())
}
