use owo_colors::OwoColorize;
use poc_framework::{keypair, RemoteEnvironment};
use poc_framework::solana_sdk::system_program;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
};
use poc_framework::random_keypair;
use poc_framework::Environment;
use poc_framework::localhost_client;
    
pub fn main() {        
    //SETUP
    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();    
    let new_account = random_keypair();

    let cliente1 = localhost_client();
    println!("New account address is: {:?}", new_account.pubkey().red());

    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000000);
        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    AccountMeta::new(new_account.pubkey(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                    ],
                    data: vec![0], 
                    }],
                    &[&keypair(3), &new_account],
                );

        env.get_account(new_account.pubkey());
    
    println!("Run the following command: ");
    println!("{} {:?}", "solana account".bold().red(), new_account.pubkey().bold().red());     
}
