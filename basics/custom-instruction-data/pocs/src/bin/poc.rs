use std::str::FromStr;
use owo_colors::OwoColorize;
use poc_framework::solana_program::pubkey::Pubkey;
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
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
pub struct InstructionData {
    name: String,
    height: u32,
}

pub fn main() {
    /*pub enum LogLevel {
        TRACE,
        DEBUG,
        INFO,
        WARN,
        ERROR,
    }
    
    setup_logging(LogLevel::DEBUG); */
        
    //SETUP
    let program = Pubkey::from_str("HZZ77M8UqthN72wewwYYmVw9WZ2w6XRgyF2V8SJ8Jn6Q").unwrap();
    let new_account = keypair(1);
    let mut cliente1 = localhost_client();

    let mut my_ser_data = InstructionData {
        name: String::from("Julio"),
        height: 10
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

    
    /*let url = "http://localhost:8899".to_string();
    let mut client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());  
    */
    println!("New account address is: {:?}", new_account.pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    AccountMeta::new(new_account.pubkey(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
            //transfer 2560 lamports from Koo3soHgwbb4Eda4kAuAFQEsHJV29MYV9VfQyzXBdqy to Koo1BQTQYawwKVBg71J2sru7W51EJgfbyyHsTFCssRW
                    }],
                    &[&keypair(3), &new_account],
                );
    /* 
    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());
     */

    let cliente1 = localhost_client();

    let my_ser_data = InstructionData {
        name: String::from("Pinky"),
        height: 2
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

    RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000000)
    .execute_as_transaction_debug(
        &[Instruction {
            program_id: program,
            accounts: vec![
                AccountMeta::new(keypair(3).pubkey(), true),
                AccountMeta::new(new_account.pubkey(), true),
                AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: my_ser_data.try_to_vec().unwrap(),  
        //transfer 2560 lamports from Koo3soHgwbb4Eda4kAuAFQEsHJV29MYV9VfQyzXBdqy to Koo1BQTQYawwKVBg71J2sru7W51EJgfbyyHsTFCssRW
                }],
                &[&keypair(3), &new_account],
            );
                   
}
