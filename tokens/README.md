# Instructions

- **mint-to** PoC is not complete, it just test some functions. 
- 	Check **mint** and **mint-2**, and finish the PoC.

# Forked from
- https://github.com/solana-developers/program-examples/

In our example, we are going to use localnet, but I wanted to keep the original README file, so, don't be confused in the next instructions and notes from the original repo.

### :warning: All token examples are on devnet!

`https://api.devnet.solana.com/`

### About Tokens

Tokens on Solana are - like everything else on Solana - accounts! They:
- are owned by the Token Program
- can only be changed by the Token Program
- have an associated Mint Authority - the only account that can mint new tokens (by calling the Token program)

How they work:   
- :apple: `Mint Account` - stores information about the token.
- :handbag: `Associated Token Account` - stores a specific balance of the Mint Account (this is essentially a counter).

> You can read all about tokens in [Solana's official SPL Token documentation](https://spl.solana.com/token).