use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    
    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    // Determine the necessary minimum rent by calculating the account's size
    //
    let account_span = instruction_data.len();
    msg!("Instruction data len: {:?}", account_span);
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    msg!("Account span: {}", &account_span);
    msg!("Lamports required: {}", &lamports_required);
    
    invoke(
        &system_instruction::create_account(
            &payer.key,
            &new_account.key,
            lamports_required,
            account_span as u64,
            &system_program::ID,
        ),
        &[
            payer.clone(), new_account.clone(), system_program.clone()
        ]
    )?;

    msg!("Account created succesfully.");
    Ok(())
}