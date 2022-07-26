#Instructions
- Although it's not necessary, we've performed the same approach, deploying the Associated token account program, using:

	`solana-test-validator --reset --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ~/	projects/program-examples/token-metadata/target/deploy/mpl_token_metadata.so --bpf-program 	ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL ~/projects/program-examples/spl-associated-	token-account/target/deploy/spl_associated_token_account.so`
	- So you can check two different ways to use it :)

- Check the flow of the PoC and how's working and interacting with the Smart Contract.
- In this example, we've try to use a lot of poc_framework functions, obtaining information about tokens, check it.
- Don't forget to run the PoC:

	`cargo run --manifest-path=./pocs/Cargo.toml --target-dir=./program/target/
	`
# Forked from
- https://github.com/solana-developers/program-examples/

# Minting an SPL Token to a Wallet

This example demonstrates how to mint an SPl Token to a Solana users's wallet.

### :key: Keys:

- The person requesting the mint must have an **associated token account** for that mint. We create this token account in the program!
- Steps:
    1. Create an associated token account for the Mint.
    2. Initialize that associated token account.
    3. Mint some amount of the Mint to the new token account.