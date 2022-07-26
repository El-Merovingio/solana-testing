# Instructions
To run, just execute:
```
chmod +x cicd.sh
./cicd.sh
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
- **Check the results with:**

`solana logs`
```
Transaction executed in slot 3324:
  Signature: Yh4z2DCVHfJubEBH2mWMTpgzAw6LB51H3yF5vaPvGXdNUKE6yF9kkZjzkQ9nUd6iWU5xAdcBBzsHZ91AWse7sD5
  Status: Ok
  Log Messages:
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE invoke [1]
    Program log: Welcome to the park, Julio!
    Program log: You are tall enough to ride this ride. Congratulations.
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE consumed 1626 of 200000 compute units
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE success

Transaction executed in slot 3325:
  Signature: 49dUhLnaym1nv8UyUr9aRZfkqL5dXCLdTgTc9wbWMBfa1PDKJzE6zoRaH95nSiSrsfhSjxDJkRWmgZtgHcVBLKH6
  Status: Ok
  Log Messages:
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE invoke [1]
    Program log: Welcome to the park, Pinky!
    Program log: You are NOT tall enough to ride this ride. Sorry mate.
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE consumed 1626 of 200000 compute units
    Program GsjzqYSMSigaha1bzi3UmnJUDxbE7ZnHDVfAwmuH2JeE success

```


# Forked from
- https://github.com/solana-developers/program-examples/


# Custom Instruction Data

Let's take a look at how to pass our own custom instruction data to a program. This data must be *serialized* to *Berkeley Packet Filter (BPF)* format - which is what the Solana runtime supports for serialized data.   

BPF is exactly why we use `cargo build-bpf` to build Solana programs in Rust. For instructions sent over RPC it's no different. We'll use a library called `borsh` on both client and program side.   

_____

**For native**, we need to add `borsh` and `borsh-derive` to `Cargo.toml` so we can mark a struct as serializable to/from **BPF format**.   

**For Anchor**, you'll see that they've made it quite easy (as in, they do all of the serializing for you).