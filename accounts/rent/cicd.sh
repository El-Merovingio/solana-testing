#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./program/target/so
solana program deploy ./program/target/so/program.so
cargo run --manifest-path=./pocs/Cargo.toml --target-dir=./program/target/
#IF error, replace the program ID in the poc.rs with the correct

echo CHECK the correct Program ID and replace 'program' variable, in poc.rs with the correct one
solana program show --programs
