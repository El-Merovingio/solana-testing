import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import * as borsh from "borsh";
import { Buffer } from "buffer";


const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


describe("mint-token", () => {

    const connection = new Connection(`http://localhost:8899/`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    class Assignable {
        constructor(properties) {
            Object.keys(properties).map((key) => {
                return (this[key] = properties[key]);
            });
        };
    };

    class TokenMetadata extends Assignable {
        toBuffer() {
            return Buffer.from(borsh.serialize(TokenMetadataSchema, this));
        }
    };

    const TokenMetadataSchema = new Map([
        [
            TokenMetadata, {
                kind: 'struct',
                fields: [
                    ['title', 'string'],
                    ['symbol', 'string'],
                    ['uri', 'string'],
                ]
            }
        ]
    ]);
  
    it("Mint!", async () => {

        const mintKeypair: Keypair = Keypair.generate();
        console.log(`New token: ${mintKeypair.publicKey}`);

        // Derive the metadata account's address and set the metadata
        //
        const metadataAddress = (await PublicKey.findProgramAddress(
            [
              Buffer.from("metadata"),
              TOKEN_METADATA_PROGRAM_ID.toBuffer(),
              mintKeypair.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ))[0];
        const metadata = new TokenMetadata({
            title: "Solana Gold",
            symbol: "GOLDSOL",
            uri: "https://github.com/solana-developers/program-examples/blob/c5b1d527ecd5f4afb4fe4c9d9b02fc2f055ff2f1/tokens/token_metadata.json",
            //uri: "https://raw.githubusercontent.com/solana-developers/program-examples/main/tokens/mint/native/assets/token_metadata.json",
        });

        // Transact with the "mint_token" function in our on-chain program
        //
        let ix = new TransactionInstruction({
            keys: [
                // Mint account
                {
                    pubkey: mintKeypair.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                // Metadata account
                {
                    pubkey: metadataAddress,
                    isSigner: false,
                    isWritable: true,
                },
                // Mint Authority
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: false,
                },
                // Rent account
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: false,
                },
                // System program
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                // Token program
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                // Token metadata program
                {
                    pubkey: TOKEN_METADATA_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: metadata.toBuffer(),
        });

        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, mintKeypair]
        );
    });
  });
  