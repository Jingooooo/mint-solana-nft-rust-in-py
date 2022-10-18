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
fn transfer_token(rpc_url: &str, secret_key : [u8; 64], to_addr : &[u8], mint_account : &[u8]) -> PyResult<String>{
    let client = RpcClient::new(rpc_url);

    let payer = Keypair::from_bytes(&secret_key).unwrap();

    let mint = Pubkey::new(mint_account);
    let wallet = Pubkey::new(to_addr);
    let assoc = get_associated_token_address(&wallet, &mint);

    let transfer_token = spl_token::instruction::transfer(
        &spl_token::ID,
        &payer.pubkey(),
        &assoc,
        &payer.pubkey(),
        &[&payer.pubkey()],
        1
    ).unwrap();

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[
            transfer_token
        ],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    let result = client
        .send_transaction(&transaction)
        .unwrap();

    Ok(result.to_string())
}

#[pyfunction]
fn thaw_account(rpc_url: &str, secret_key : [u8; 64], to_addr : &[u8], mint_account : &[u8]) -> PyResult<String>{
    let client = RpcClient::new(rpc_url);

    let payer = Keypair::from_bytes(&secret_key).unwrap();

    let mint = Pubkey::new(mint_account);
    let wallet = Pubkey::new(to_addr);
    let assoc = get_associated_token_address(&wallet, &mint);

    let thaw_account = spl_token::instruction::thaw_account(
        &spl_token::ID,
        &assoc,
        &mint,
        &payer.pubkey(),
        &[&payer.pubkey()],
    )
    .unwrap();

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[
            thaw_account
        ],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    let result = client
        .send_transaction(&transaction)
        .unwrap();

    println!("NFT Tokens minted successfully.");

    Ok(result.to_string())
}


#[pyfunction]
fn mint_and_freeze(rpc_url : &str, secret_key : [u8; 64], to_addr : &[u8], _uri : String) -> PyResult<String> {
    let client = RpcClient::new(rpc_url);
    // let client = RpcClient::new("https://api.devnet.solana.com");

    let payer = Keypair::from_bytes(&secret_key).unwrap();
    let mint_account = Keypair::new();
    let mint_authority_account = &payer;
    let metadata_account = find_metadata_account(&mint_account.pubkey()).0;
    let master_edition_account = find_master_edition_account(&mint_account.pubkey()).0;

    let name = "TESTOKEN";
    let symbol = "TTT";
    let uri = _uri;
    let seller_fee_basis_point: u16 = 100;
    let creators = vec![Creator {
        address: payer.pubkey(),
        verified: true,
        share: 100,
    }];

    // 7Ew4GGk5pVbnwXbxT3UyeZb8BMSsg8oS4CpTcTy8Mv4f
    let master_nft_pubkey: [u8; 32] = [
        92, 183, 203, 47, 42, 53, 64, 169, 31, 216, 5, 67, 47, 7, 64, 92, 21, 9, 81, 100, 241, 96,
        37, 131, 60, 113, 78, 26, 251, 158, 192, 216,
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
        Some(&payer.pubkey()),
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

    let freeze_account = spl_token::instruction::freeze_account(
        &spl_token::ID,
        &assoc,
        &mint_account.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
    )
    .unwrap();

    // let thaw_account = spl_token::instruction::thaw_account(
    //     &spl_token::ID,
    //     &assoc,
    //     &mint_account.pubkey(),
    //     &wallet.pubkey(),
    //     &[&wallet.pubkey()],
    // )
    // .unwrap();

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[
            create_account_instruction,
            initialize_mint_instruction,
            create_assoc_instruction,
            mint_token_to,
            create_metadata_account,
            freeze_account,
            create_master_edition,
            set_collection,
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );
    let result = client
        .send_transaction(&transaction)
        .unwrap();

    // println!("Signature : {}", result.to_string());
    println!("NFT Tokens minted successfully.");

    Ok(result.to_string())
}

#[pymodule]
fn rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(transfer_token, m)?)?;
    m.add_function(wrap_pyfunction!(thaw_account, m)?)?;
    m.add_function(wrap_pyfunction!(mint_and_freeze, m)?)?;
//     m.add_function(wrap_pyfunction!(send_transaction, m)?)?;

    Ok(())
}
