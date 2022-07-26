use owo_colors::OwoColorize;
use poc_framework::{keypair, RemoteEnvironment,};
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
};

use poc_framework::Environment;
use poc_framework::localhost_client;
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
    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    let new_account = keypair(1);
    let cliente1 = localhost_client();

    let my_ser_data = CarnivalInstructionData {
        name: String::from("Julio"),
        height: 48,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Scrambler")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

    println!("Payer: {:?}", keypair(3).pubkey().red());

    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000);
        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );


    let my_ser_data = CarnivalInstructionData {
        name: String::from("Javier"),
        height: 36,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Scrambler")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();

    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );


    let my_ser_data = CarnivalInstructionData {
        name: String::from("Niky"),
        height: 55,
        ticket_count: 5,
        attraction: String::from("ride"),
        attraction_name: String::from("Ferris Wheel")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

    let my_ser_data = CarnivalInstructionData {
        name: String::from("Chelo"),
        height: 58,
        ticket_count: 3,
        attraction: String::from("ride"),
        attraction_name: String::from("Tilt-a-Whirl")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );
                
//GAME
    let my_ser_data = CarnivalInstructionData {
        name: String::from("Pinky"),
        height: 36,
        ticket_count: 15,
        attraction: String::from("game"),
        attraction_name: String::from("I Got It!")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

        
    let my_ser_data = CarnivalInstructionData {
        name: String::from("Cachimba"),
        height: 52,
        ticket_count: 3,
        attraction: String::from("game"),
        attraction_name: String::from("Ring Toss")
    };
    
    let mut my_data: Vec<u8> = vec![];
    my_ser_data.serialize(&mut my_data).unwrap();
    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
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
    println!("Payer: {:?}", keypair(3).pubkey().red());

        env.execute_as_transaction_debug(
            &[Instruction {
                program_id: programa,
                accounts: vec![
                    AccountMeta::new(keypair(3).pubkey(), true),
                    ],
                    data: my_ser_data.try_to_vec().unwrap(),  
                    }],
                    &[&keypair(3)],
                );

}
