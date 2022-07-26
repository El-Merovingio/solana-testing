use owo_colors::OwoColorize;
use poc_framework::{keypair, RemoteEnvironment};
use poc_framework::solana_sdk::system_program;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
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
    //SETUP
    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    let new_account = keypair(1);
    let cliente1 = localhost_client();

    let my_ser_data = InstructionData {
        name: String::from("Julio"),
        height: 10
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

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
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3), &new_account],
                );

    let my_ser_data = InstructionData {
        name: String::from("Pinky"),
        height: 2
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    AccountMeta::new(new_account.pubkey(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3), &new_account],
                );
                    
}
