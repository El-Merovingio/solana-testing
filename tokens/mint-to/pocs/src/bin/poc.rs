use std::{str::FromStr};
use owo_colors::OwoColorize;
use poc_framework::solana_program::pubkey::Pubkey;
use poc_framework::{keypair, RemoteEnvironment,};
use poc_framework::solana_sdk::system_program;
use poc_framework::random_keypair;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
};

use poc_framework::Environment;
use poc_framework::localhost_client;
//use poc_framework::LogLevel::DEBUG;

use { 
    poc_framework::spl_token::{
      state::Account as TokenAccount
    },
// not necessary to use here, we are going to use the mpl token program address
//    mpl_token_metadata::{
//        instruction as mpl_instruction,
//    },
};
use borsh::{BorshSerialize, BorshDeserialize};

// We use the same Structure created in the Smart Contract state/mintrs
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    title: String,
    symbol: String,
    uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintTokenTo {
    pub amount: u64,
}

pub fn main() {

    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    //let programa = Pubkey::from_str("PUT_HERE_THE_PROGRAM_ID").unwrap();
    //mpl token program address
    let mpl_token_metadata = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let cliente1 = localhost_client();
    let mint_account = random_keypair();
    let mint_authority =  keypair(2);
    // exammple:
    // https://github.com/solana-developers/program-examples/blob/c5b1d527ecd5f4afb4fe4c9d9b02fc2f055ff2f1/tokens/token_metadata.json
    // We use the same Structure created in the Smart Contract
    let metadata = TokenMetadata {
        title: String::from("Solana Gold"),
        symbol: String::from("GOLDSOL"),
        uri: String::from("https://images.all-free-download.com/images/graphiclarge/solana_coin_sign_icon_shiny_golden_symmetric_geometrical_design_6919941.jpg"),
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data: Vec<u8> = vec![];
    metadata.serialize(&mut my_data).unwrap();

    //PDA create
    //https://github.com/alexlu0917/SolanaNFTStaking/blob/99684756c661bfe78e22d296ac01d0d74e6779ac/client/src/main.rs
    //let nft = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
        // Avoid this.... 
        // "Program log:  Metadata's key must match seed of ['metadata', program id, mint] provided",
        // Check docs: https://docs.metaplex.com/programs/token-metadata/instructions
        "metadata".as_bytes(),
        &mpl_token_metadata.to_bytes(),
        &mint_account.pubkey().to_bytes(),
        //  &[0]
        ],
        &mpl_token_metadata,
    );

    println!("{:} {:?}", "METADATA PDA Address: ".bold().blue(), metadata_pda.blue());

    
    /*let url = "http://localhost:8899".to_string();
    let mut client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());  
    */

        //mint_authority = keypair(2)
    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(2), 10000000000);
            env.execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), true),
                        AccountMeta::new(metadata_pda, false),
                        AccountMeta::new_readonly(mint_authority.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(mpl_token_metadata, false),
                        ],
                        data: metadata.try_to_vec().unwrap(),  
                        }],
                        &[&mint_account, &mint_authority],
                    );

    let metadata2 = MintTokenTo {
        amount: 1,
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data2: Vec<u8> = vec![];
    metadata2.serialize(&mut my_data2).unwrap();

    let token_acc = poc_framework::spl_associated_token_account::get_associated_token_address(
        &mint_authority.pubkey(), 
        &mint_account.pubkey(),
        );

            env.execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), true),
                        AccountMeta::new(token_acc, false),
                        AccountMeta::new_readonly(mint_authority.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_associated_token_account::ID, false),
                        ],
                        data: metadata2.try_to_vec().unwrap(),  
                        }],
                        &[&mint_account, &mint_authority],
                    );

    let mint_account_info = env.get_account(mint_account.pubkey());
    let mint_authority_info = env.get_account(mint_authority.pubkey());

    println!("Mint account info: {:?}", mint_account_info.unwrap().blue());
    println!("");

    println!("Mint authority info: {:?}", mint_authority_info.unwrap().red());
    println!("");

    let token_acc_token = env.get_unpacked_account::<TokenAccount>(token_acc);

    println!("Token account address: {:?}", token_acc.blue());
    println!("");
    println!("Token account unpacked info: {:?}", token_acc_token.unwrap().blue());   
    println!("");

}
