# Instructions
Because we are patching the source **crates-io** with a local checked out version of solana-program, solana-bpf-loader, spl-token and specially the metaplex token metadata program (mpl-token-metadata) to the crates.io registry for these local packages, our approach will be the following:
- We need to download and compile the mpl-token-metadata and deploy it in our local environment. To do this, we need to add the compiled BPF mpl token program to the genesis configuration, using the "real" mpl address which is **metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ** and the .so (compiled).
- Let's imagine we already compiled the mpl token metadata program in **~/projects/ad-hoc/target/programs/mpl_token_metadata.so**, so, we need to run the following command:

	`solana-test-validator --reset --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ~/projects/ad-hoc/target/programs/mpl_token_metadata.so
`

- If we want to log the solana-test-validator we can run the same command (see above) adding the **--log** flag, or running solana-test-validator --log into another window.

**Notes**
- If you can't get and / or compile the mpl token metadata program, check the metaplex Git: https://github.com/metaplex-foundation/metaplex-program-library
- More information about the crates dependencies here: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
- At the time I created this repo, the "mint" program was removed from the original git (mentioned below), I'm assuming we are going to need the asset token_metadata json structure, and the test yarn code, so, that's why those were included in this repo. If you want to use the test.js code, you need to install the dependencies with npm (npm i @solana/web3.js @solana/spl-token borsh buffer)

# Commands summary
- Install rust and solana
- To run, just execute:
```
chmod +x cicd.sh
./cicd.sh
```

- The following point, was to avoid errors using **poc_framework 0.1.6**, but we have updated the dependencies, to use **poc_framework 0.2.0**.
- Use rustup 1.60 version, check this: https://github.com/neodyme-labs/solana-poc-framework/issues/9
- Compile the Smart Contract:
	`cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./program/target/so
`
- Copy the mpl source:
	`cp -R ~/.cargo/registry/src/github.com-1ecc6299db9ec823/mpl-token-metadata-1.3.3 .`
- Compile it:
	`cargo build-bpf --manifest-path=./mpl-token-metadata-1.3.3/Cargo.toml --bpf-out-dir=./program/target/so
`
- Run the thest validator, resetting the config, and using the **--log** flag:
	`solana-test-validator --reset --log --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ./program/target/so/mpl_token_metadata.so
`
- In another window, check if the solana account exists:
`solana account metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s|more
`
	- Output, something like this:
	```	Public Key: metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s
	Balance: 0.000000001 SOL
	Owner: BPFLoader2111111111111111111111111111111111
	Executable: true
	Rent Epoch: 0
	Length: 384376 (0x5dd78) bytes
	0000:   7f 45 4c 46  02 01 01 00  00 00 00 00  00 00 00 00   .ELF............
	0010:   03 00 f7 00  01 00 00 00  88 ed 02 00  00 00 00 00   ................
	0020:   40 00 00 00  00 00 00 00  38 db 05 00  00 00 00 00   @.......8.......
	..........
	etc.....
	.......
	```
- Compile the Smart Contract (if we didn't before)
	`cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./program/target/so
	`
- Deploy the Smart Contract and get the program-id (will be used in the PoC)
	`solana program deploy ./program/target/so/program.so`

- Replace the address used in the PoC code with the corresponding program-id

```
	............
	.......   
	let programa = Pubkey::from_str("PUT_HERE_THE_PROGRAM_ID").unwrap();
	....
	............
```
- Execute the PoC code.... There are exploitable vulns??? :)

# Troubleshooting
- If we got an **already in use** error message, i.e.
`                            "Create Account: account Address { address: 	Koo1BQTQYawwKVBg71J2sru7W51EJgfbyyHsTFCssRW, base: None } already in use",
	`
	We need to reset the solana test validator, deploy again the Smart contract and perform the PoC test, this is because, POC framework uses "static" addresses in its structure.
- Another option could be, change the keypair number, i.e.
```
	........
	.....
    let mint_account = keypair(10);
    let mint_authority =  keypair(11);
	.....
	.........
    RemoteEnvironment::new_with_airdrop(cliente1, keypair(11), 10000000000)
	......
	........
```

# Forked from
- https://github.com/solana-developers/program-examples/

# Create a New SPL Token Mint

This example demonstrates how to create an SPl Token on Solana with some metadata such as a token symbol and icon.

### :key: Keys:

- SPL Tokens by default have **9 decimals**, and **NFTs have 0 decimals**. "Decimals" here means the number of decimal; ie. a token with 3 decimals will be tracked in increments of 0.001.   
- You can use [Metaplex's Token Metadata Program](https://docs.metaplex.com/) to create metadata for your token.
- Steps:
    1. Create an account for the Mint.
    2. Initialize that account as a Mint Account.
    3. Create a metadata account associated with that Mint Account.