import { Connection, Keypair, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';
import {
    createCreateMetadataAccountV3Instruction,
    CreateMetadataAccountArgsV3,
} from '@metaplex-foundation/mpl-token-metadata';
import * as fs from 'fs';
import { Buffer } from 'buffer';

(async () => {
    const connection = new Connection("https://api.devnet.solana.com");

    // Load the update authority keypair
    const keypairPath = "/Users/copycoder/metaloot-keypair.json";
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
    const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));

    console.log("Payer Public Key:", payer.publicKey.toBase58());

    // Mint address of your token
    const mintAddress = new PublicKey("78BN2F4urNgtVko3vCaVB7KadfPmc7XZ9XR9at5fWSMV");

    // Metadata program ID
    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // Derive the metadata PDA
    const [metadataPDA] = await PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mintAddress.toBuffer(),
        ],
        METADATA_PROGRAM_ID
    );

    console.log("Metadata PDA:", metadataPDA.toBase58());

    // Check if metadata account exists
    const metadataAccountInfo = await connection.getAccountInfo(metadataPDA);
    if (metadataAccountInfo) {
        console.log("Metadata already exists. You can update it.");
        return;
    } else {
        console.log("No metadata found. Creating metadata...");
    }

    // Metadata fields
    const metadataArgs: CreateMetadataAccountArgsV3 = {
        data: {
            name: "MetaLoot",
            symbol: "LOOT",
            uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot.json",
            sellerFeeBasisPoints: 500, // Example: 5% royalties
            creators: [
                {
                    address: payer.publicKey.toBase58(),
                    verified: true,
                    share: 100,
                },
            ],
        },
        isMutable: true,
        collection: null,
        uses: null,
    };

    try {
        // Create metadata account instruction
        const createInstruction = createCreateMetadataAccountV3Instruction(
            {
                metadata: metadataPDA,
                mint: mintAddress,
                mintAuthority: payer.publicKey,
                payer: payer.publicKey,
                updateAuthority: payer.publicKey,
            },
            {
                createMetadataAccountArgsV3: metadataArgs,
            }
        );

        // Create the transaction
        const transaction = new Transaction().add(createInstruction);
        transaction.feePayer = payer.publicKey;
        transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

        // Sign and send the transaction
        transaction.sign(payer);

        const signature = await connection.sendTransaction(transaction, [payer]);
        await connection.confirmTransaction(signature, "confirmed");
        console.log("Metadata created successfully. Transaction Signature:", signature);
    } catch (error) {
        console.error("Error creating metadata account:", error);
    }
})();
