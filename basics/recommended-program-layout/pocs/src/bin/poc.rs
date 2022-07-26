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
use borsh::{BorshSerialize, BorshDeserialize};

//Same struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CarnivalInstructionData {
    pub name: String,
    pub height: u32,
    pub ticket_count: u32,
    pub attraction: String,
    pub attraction_name: String,
}

pub fn main() {
    let program = Pubkey::from_str("EFywtcMaAqJGL6kVZ9oUpUy2SWiJCez55DdUf8deo8Xq").unwrap();
    let new_account = keypair(1);
    let mut cliente1 = localhost_client();

    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Julio"),
        height: 48,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Scrambler")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );


    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Javier"),
        height: 36,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Scrambler")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let mut cliente1 = localhost_client();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );


    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Niky"),
        height: 55,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Ferris Wheel")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let mut cliente1 = localhost_client();
    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Chelo"),
        height: 58,
        ticket_count: 3,
        attraction: String::from("ride"),
        attraction_name: String::from("Tilt-a-Whirl")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let mut cliente1 = localhost_client();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );
                
//GAME
    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Pinky"),
        height: 36,
        ticket_count: 15,
        attraction: String::from("game"),
        attraction_name: String::from("I Got It!")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let mut cliente1 = localhost_client();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

        
    let mut my_ser_data = CarnivalInstructionData {
        name: String::from("Cachimba"),
        height: 52,
        ticket_count: 3,
        attraction: String::from("game"),
        attraction_name: String::from("Ring Toss")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let mut cliente1 = localhost_client();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

//FOOD
    let my_ser_data = CarnivalInstructionData {
        name: String::from("Chelo"),
        height: 58,
        ticket_count: 3,
        attraction: String::from("food"),
        attraction_name: String::from("Larry's Pizza")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    let cliente1 = localhost_client();

    println!("Payer: {:?}", keypair(3).pubkey().red());

RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000)
        .execute_as_transaction_debug(
            &[Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

}
