use mpl_token_metadata::{instruction, pda::find_metadata_account, state::*};
use pyo3::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, program_pack::Pack, pubkey::Pubkey, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use spl_token::state::Mint;
use mpl_token_metadata::pda::find_master_edition_account;
use spl_associated_token_account::get_associated_token_address;

#[pyfunction]
fn send_transaction(_uri : String, to_addr : &[u8]) {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let secret_key: [u8; 64] = [
        26, 110, 108, 237, 157, 77, 56, 250, 82, 157, 243, 112, 123, 113, 249, 246, 96, 123, 169,
        28, 58, 41, 142, 205, 31, 28, 95, 1, 146, 95, 101, 82, 124, 168, 12, 182, 64, 235, 134,
        255, 216, 169, 149, 158, 128, 248, 2, 4, 23, 122, 107, 209, 217, 109, 165, 146, 142, 242,
        177, 246, 47, 42, 186, 68,
    ];


    let payer = Keypair::from_bytes(&secret_key).unwrap();
    let mint_account = Keypair::new();
    let mint_authority_account = &payer;
    let metadata_account = find_metadata_account(&mint_account.pubkey()).0;
    let master_edition_account = find_master_edition_account(&mint_account.pubkey()).0;


    let name = "asdfwer";
    let symbol = "TTT";
    let uri = _uri;
    let seller_fee_basis_point: u16 = 100;
    let creators = vec![Creator {
        address: payer.pubkey(),
        verified: true,
        share: 100,
    }];

    // 6sDueq754X8Pm1bb5ubSYHW7xEPKPxD9BxoZksENqAke
    let master_nft_pubkey: [u8; 32] = [
        87, 40, 17, 234, 23, 122, 159, 21, 168, 182, 201, 193, 251, 14, 11, 118, 175, 147, 205,
        142, 117, 99, 118, 174, 230, 165, 191, 134, 253, 98, 7, 23,
    ];

    let collections = Collection{
        verified: false,
        key : Pubkey::from(master_nft_pubkey)
    };

    let decimals: u8 = 0;

    let space = Mint::LEN;

    let minimum_balance_for_rent_exemption = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    let wallet = Pubkey::new(to_addr);
    let assoc = get_associated_token_address(&wallet, &mint_account.pubkey());

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        minimum_balance_for_rent_exemption,
        space as u64,
        &spl_token::ID,
    );

    let initialize_mint_instruction: Instruction = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint_account.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )
    .unwrap();

    let create_assoc_instruction = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &wallet,
        &mint_account.pubkey(),
        &spl_token::ID,
    );

    let mint_token_to = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint_account.pubkey(),
        &assoc,
        &payer.pubkey(),
        &[&payer.pubkey()],
        1,
    )
    .unwrap();

    let create_metadata_account = instruction::create_metadata_accounts_v2(
        mpl_token_metadata::ID,
         metadata_account,
        mint_account.pubkey(),
        mint_authority_account.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
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

    let create_master_edition = instruction::create_master_edition_v3(
        mpl_token_metadata::ID,
        master_edition_account,
        mint_account.pubkey(),
        payer.pubkey(),
        mint_authority_account.pubkey(),
        metadata_account,
        payer.pubkey(),
        Some(1),
    );

    let set_collection = instruction::set_and_verify_sized_collection_item(
        mpl_token_metadata::ID,
        metadata_account,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        collections.key,
        find_metadata_account(&collections.key).0,
        find_master_edition_account(&collections.key).0,
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
            create_master_edition,
            set_collection
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
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
