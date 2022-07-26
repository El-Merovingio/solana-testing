use std::str::FromStr;
use owo_colors::OwoColorize;
use poc_framework::solana_program::pubkey::Pubkey;
use poc_framework::solana_program::sysvar::rent;
use poc_framework::{keypair, RemoteEnvironment, setup_logging, solana_program};
use poc_framework::solana_sdk::system_program;
use poc_framework::solana_client::rpc_client::RpcClient;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
};

use poc_framework::Environment;
use poc_framework::localhost_client;
//use poc_framework::LogLevel::DEBUG;

use { 
    poc_framework::spl_token::{
        instruction as token_instruction,
        state::Account as TokenAccount
    },

// not necessary to use here, we are going to use the mpl token program address
//    mpl_token_metadata::{
//        instruction as mpl_instruction,
//    },
};

use poc_framework::spl_associated_token_account::get_associated_token_address;

use borsh::{BorshSerialize, BorshDeserialize};

// We use the same Structure created in the Smart Contract
// from state/mint_state.rs
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    pub title: String,
    pub symbol: String,
    pub uri: String,
    pub mint_authority_pda_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintTokensTo {
    pub amount: u64,
    pub mint_authority_pda_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransferTokensTo {
    pub amount: u64,
}

pub fn main() {

    let programa = Pubkey::from_str("PUT_HERE_THE_PROGRAM_ID").unwrap();
    //mpl token program address
    let mpl_token_metadata = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    /*export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
    from https://github.com/solana-labs/solana-program-library/blob/48fbb5b7/token/js/src/constants.ts#L7 */
    let associated_token_program = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
    let cliente1 = localhost_client();
    let mint_account = keypair(1);
    let mint_authority =  keypair(2);

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

    let (mint_auth_pda, mint_auth_bump) = Pubkey::find_program_address(
        &[
            "mint_authority_".as_bytes(),
            &mint_account.pubkey().to_bytes(),
        ],
        &programa,
    );

    println!("Mint Auth pda addr: {:?}", mint_auth_pda);
    println!("Mint Auth bump: {:?}", mint_auth_bump);

    // exammple:
    // https://github.com/solana-developers/program-examples/blob/c5b1d527ecd5f4afb4fe4c9d9b02fc2f055ff2f1/tokens/token_metadata.json
    // We use the same Structure created in the Smart Contract
    let metadata = TokenMetadata {
        title: String::from("Solana Gold"),
        symbol: String::from("GOLDSOL"),
        uri: String::from("https://images.all-free-download.com/images/graphiclarge/solana_coin_sign_icon_shiny_golden_symmetric_geometrical_design_6919941.jpg"),
        mint_authority_pda_bump: mint_auth_bump
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data: Vec<u8> = vec![];
    metadata.serialize(&mut my_data).unwrap();
    
    /*let url = "http://localhost:8899".to_string();
    let mut client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());  
    */

        //mint_authority = keypair(2)
    RemoteEnvironment::new_with_airdrop(cliente1, keypair(2), 10000000000)
            .execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), true),
                        AccountMeta::new(mint_auth_pda, false),
                        AccountMeta::new(metadata_pda, false),
                        AccountMeta::new(mint_authority.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(mpl_token_metadata, false),
                        ],
                        data: metadata.try_to_vec().unwrap(),  
                        }],
                        &[&mint_account, &mint_authority],
                    );
    
    let metadata2 = MintTokensTo {
        amount: 1,
        mint_authority_pda_bump: mint_auth_bump
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data2: Vec<u8> = vec![];
    metadata2.serialize(&mut my_data2).unwrap();

    // https://docs.rs/spl-associated-token-account/latest/spl_associated_token_account/fn.get_associated_token_address.html
    /*
    pub fn get_associated_token_address(
    wallet_address: &Pubkey, 
    spl_token_mint_address: &Pubkey
    ) -> Pubkey
    */
    let token_dir = poc_framework::spl_associated_token_account::get_associated_token_address(
                    &mint_authority.pubkey(), 
                    &mint_account.pubkey(),
                    );
    
    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    //RemoteEnvironment::new_with_airdrop(cliente1, keypair(2), 10000000000)
    RemoteEnvironment::new(cliente1, keypair(2))
            .execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), false),
                        AccountMeta::new_readonly(mint_auth_pda, false),
                        AccountMeta::new(token_dir, false),
                        AccountMeta::new(mint_authority.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(associated_token_program, false),
                        ],
                        data: metadata2.try_to_vec().unwrap(),  
                        }],
                        &[&mint_authority],
                    );

    println!("Token address: {:?}", token_dir.blue());
    println!("");

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let token_addr = RemoteEnvironment::new(cliente1, keypair(2))
            .get_account(token_dir);

    println!("Token without unwrap: {:?}", token_addr.blue());
    println!("");
    let token_unwrap = token_addr.unwrap();
    println!("Token using unwrap: {:?}", token_unwrap.red());
    println!("");

    let token_unwrap_data = &token_unwrap.data;
    println!("Token data: {:?}", token_unwrap_data.blue().bold());
    println!("");

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let token_info = RemoteEnvironment::new(cliente1, keypair(2))
            .get_unpacked_account::<TokenAccount>(token_dir);

    println!("Token unpacked account: {:?}", token_info.blue());       
    println!("Token unpacked account using unwrap: {:?}", token_info.unwrap().red());
    println!("");

    let metadata3 = TransferTokensTo {
        amount: 1,
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data3: Vec<u8> = vec![];
    metadata3.serialize(&mut my_data3).unwrap();

    let receiver = keypair(5);

    let owner_token = poc_framework::spl_associated_token_account::get_associated_token_address(
                    &mint_authority.pubkey(), 
                    &mint_account.pubkey(),
                    );

    
    let receiver_token = poc_framework::spl_associated_token_account::get_associated_token_address(
                    &receiver.pubkey(), 
                    &mint_account.pubkey(),
                    );
    
    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    RemoteEnvironment::new_with_airdrop(cliente1, keypair(5), 10000000000)
            .execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), false),
                        AccountMeta::new(owner_token, false),
                        AccountMeta::new(receiver_token, false),
                        AccountMeta::new(mint_authority.pubkey(), true),
                        AccountMeta::new(receiver.pubkey(), true),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_associated_token_account::ID, false),
                        ],
                        data: metadata3.try_to_vec().unwrap(),  
                        }],
                        &[&mint_authority, &receiver],
                    );

                    
    println!("Owner Token address: {:?}", owner_token.blue());
    println!("");

    println!("Receiver Token address: {:?}", receiver_token.blue());
    println!("");

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let owner_token_addr = RemoteEnvironment::new(cliente1, keypair(2))
            .get_account(owner_token);

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let receiver_token_addr = RemoteEnvironment::new(cliente1, keypair(2))
            .get_account(receiver_token);

    println!("Owner token info: {:?}", owner_token_addr.unwrap().blue());
    println!("");

    println!("Receiver token info: {:?}", receiver_token_addr.unwrap().red());
    println!("");

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let owner_token_unpack = RemoteEnvironment::new(cliente1, keypair(2))
            .get_unpacked_account::<TokenAccount>(owner_token);

    let cliente1 = localhost_client();
    //mint_authority = keypair(2)
    let receiver_token_unpack = RemoteEnvironment::new(cliente1, keypair(2))
            .get_unpacked_account::<TokenAccount>(receiver_token);

    println!("Owner Token unpacked account: {:?}", owner_token_unpack.unwrap().blue());   
    println!("");    
    println!("Receiver Token unpacked account: {:?}", receiver_token_unpack.unwrap().red());
    println!("");

}
