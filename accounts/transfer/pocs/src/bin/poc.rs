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
    let program = Pubkey::from_str("EWv1J8ssDePevka4bj1Yioja3cEcAThp4dH43afbABW").unwrap();
    
    let new_account = keypair(1);

    let cliente1 = localhost_client();
    
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
                    data: vec![0,100,0,0,0,0,0,0],  
            //transfer 2560 lamports from Koo3soHgwbb4Eda4kAuAFQEsHJV29MYV9VfQyzXBdqy to Koo1BQTQYawwKVBg71J2sru7W51EJgfbyyHsTFCssRW
                    }],
                    &[&keypair(3), &new_account],
                );
    /* 
    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());
     */
    let mut cliente2 = localhost_client();
    let mut env = RemoteEnvironment::new(cliente2, keypair(3))
        .get_account(new_account.pubkey());

    println!("Get account. Address: {}, Data: {:?}", new_account.pubkey().bold(), env);

    let cliente2 = localhost_client();
    let env = RemoteEnvironment::new(cliente2, keypair(3))
        .get_account(keypair(3).pubkey());

    println!("Get account. Address: {}, Data: {:?}", keypair(3).pubkey().bold(), env);

                   
}
