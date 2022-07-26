# Instructions
To run, just execute:
```
chmod +x cicd.sh
/cicd.sh
```

The old fashion way is the following:
It's recommended to deploy the program running the following.
- First you must build the BPF:
> cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./program/target/so
- Then deploy the program and copy the Program id:
> solana program deploy ./program/target/so/program.so
	- :code: If the :code:
- Check and update the '[patch.crates-io]' in Cargo.toml in pocs/
- Replace the program in poc.rs with the copied Program id:
> let program = Pubkey::from_str("~~HZZ77M8UqthN72wewwYYmVw9WZ2w6XRgyF2V8SJ8Jn6Q~~").unwrap();
- Once replaced, run the poc program:
>  cargo run --manifest-path=./pocs/Cargo.toml --target-dir=./program/target/


# Forked from
- https://github.com/solana-developers/program-examples/

# Create Account

:wrench: We're going to create a Solana account. :wrench:   
This account is going to be a **system account** - meaning it will be owned by the System Program. In short, this means only the System Program will be allowed to modify it's data.   
   
In this example, this account will simply hold some SOL.

### Links:
- [Solana Cookbook - How to Create a System Account](https://solanacookbook.com/references/accounts.html#how-to-create-a-system-account)
- [Rust Docs - solana_program::system_instruction::create_account](https://docs.rs/solana-program/latest/solana_program/system_instruction/fn.create_account.html)
