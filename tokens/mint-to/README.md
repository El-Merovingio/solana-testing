# Instructions
It's almost the same "idea" as the last one Smart Contract, but with another layout approach.

Because we are patching the source **crates-io** with a local checked out version of solana-program, solana-bpf-loader, spl-token and specially the metaplex token metadata program (mpl-token-metadata) to the crates.io registry for these local packages, our approach will be the following:
- We need to download and compile the mpl-token-metadata and deploy it in our local environment. To do this, we need to add the compiled BPF mpl token program to the genesis configuration, using the "real" mpl address which is **metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ** and the .so (compiled).
- Let's imagine we already compiled the mpl token metadata program in **~/projects/ad-hoc/target/programs/mpl_token_metadata.so**, so, we need to run the following command:

	`solana-test-validator --reset --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ~/projects/ad-hoc/target/programs/mpl_token_metadata.so
`

- If we want to log the solana-test-validator we can run the same command (see above) adding the **--log** flag, or running solana-test-validator --log into another window.

- Don't forget to run the PoC:

	`cargo run --manifest-path=./pocs/Cargo.toml --target-dir=./program/target/
	`

**Notes**
- If you can't get and / or compile the mpl token metadata program, check the metaplex Git: https://github.com/metaplex-foundation/metaplex-program-library
- More information about the crates dependencies here: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
- At the time I created this repo, the "mint" program was removed from the original git (mentioned below), I'm assuming we are going to need the asset token_metadata json structure, and the test yarn code, so, that's why those were included in this repo. If you want to use the test.js code, you need to install the dependencies with npm (npm i @solana/web3.js @solana/spl-token borsh buffer)

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