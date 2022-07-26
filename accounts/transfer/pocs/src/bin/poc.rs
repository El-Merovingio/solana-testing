use owo_colors::OwoColorize;
use poc_framework::{keypair, RemoteEnvironment};
use poc_framework::solana_sdk::system_program;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
};
use poc_framework::Environment;
use poc_framework::localhost_client;
    
pub fn main() {
    //SETUP
    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    let new_account = keypair(1);
    let cliente1 = localhost_client();
    
    println!("New account address is: {:?}", new_account.pubkey().red());

    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000000);
        env.airdrop(new_account.pubkey(), 100000000);
        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
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
 
        let acc_info = env.get_account(new_account.pubkey()).unwrap();
    println!("Get new_account lamports: {:?}", acc_info.lamports.bold());
        
        let acc_info_2 = env.get_account(keypair(3).pubkey()).unwrap();
    println!("Get account (3) lamports: {}", acc_info_2.lamports.bold());                   
}
