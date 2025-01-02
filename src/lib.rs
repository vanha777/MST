use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs, SetAndVerifyCollection, UpdateMetadataAccountV2,
        UpdateMetadataAccountV2InstructionArgs, UpdateV1,
    },
    types::{
        CollectionDetails, CollectionDetailsToggle, CollectionToggle, Creator, Data, DataV2,
        RuleSetToggle, UsesToggle,
    },
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, EncodableKey, Signer},
    transaction::Transaction,
};
use std::{error::Error, str::FromStr};
pub fn create_metadata(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn Error>> {
    // Define metadata arguments
    let metadata_args = CreateMetadataAccountV3InstructionArgs {
        // name: "MetaLoot".to_string(),
        // symbol: "MTL".to_string(),
        // uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot.json".to_string(),
        // seller_fee_basis_points: 500, // Set your seller fee basis points
        // creators: Some(vec![Creator {
        //     address: payer.pubkey(),
        //     verified: true,
        //     share: 100, // 100% share
        // }]),
        data: DataV2{
            name: "MetaLoot Token".to_string(),
            symbol:  "MTL".to_string(),
            uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot.json".to_string(),
            seller_fee_basis_points: 0,
         creators: Some(vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100, // 100% share
        }]),
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    };

    // Create the instruction
    let instruction = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: mint_address,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: solana_sdk::system_program::ID,
        rent: Some(solana_sdk::sysvar::rent::ID),
    }
    .instruction(metadata_args);

    // Get recent blockhash
    let blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    // Send and confirm transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Metadata account created. Signature: {}", signature);

    Ok(())
}

pub fn create_metadata_nfts_collection(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn Error>> {
    // Define metadata arguments
    let metadata_args = CreateMetadataAccountV3InstructionArgs {
        // name: "MetaLoot".to_string(),
        // symbol: "MTL".to_string(),
        // uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot.json".to_string(),
        // seller_fee_basis_points: 500, // Set your seller fee basis points
        // creators: Some(vec![Creator {
        //     address: payer.pubkey(),
        //     verified: true,
        //     share: 100, // 100% share
        // }]),
        data: DataV2{
            name: "MetaForge Rewards Collection".to_string(),
            symbol:  "MFR".to_string(),
            uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/Non-FungubleTokenCol.json".to_string(),
            seller_fee_basis_points: 0,
         creators: Some(vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100, // 100% share
        }]),
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: Some(CollectionDetails::V1 { size: 100 }),
    };

    // Create the instruction
    let instruction = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: mint_address,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: solana_sdk::system_program::ID,
        rent: Some(solana_sdk::sysvar::rent::ID),
    }
    .instruction(metadata_args);

    // Get recent blockhash
    let blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    // Send and confirm transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Metadata account created. Signature: {}", signature);

    Ok(())
}

pub fn create_metadata_nfts(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn Error>> {
    // // Define metadata arguments
    // let metadata_args = CreateMetadataAccountV3InstructionArgs {
    //     // name: "MetaLoot".to_string(),
    //     // symbol: "MTL".to_string(),
    //     // uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot.json".to_string(),
    //     // seller_fee_basis_points: 500, // Set your seller fee basis points
    //     // creators: Some(vec![Creator {
    //     //     address: payer.pubkey(),
    //     //     verified: true,
    //     //     share: 100, // 100% share
    //     // }]),
    //     data: DataV2{
    //         name: "MetaLian First Lander".to_string(),
    //         symbol:  "MFL".to_string(),
    //         uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/Non-FungubleToke.json".to_string(),
    //         seller_fee_basis_points: 600,
    //      creators: Some(vec![Creator {
    //         address: payer.pubkey(),
    //         verified: true,
    //         share: 100, // 100% share
    //     }]),
    //         collection: Some(mpl_token_metadata::types::Collection {
    //             verified: false,
    //             // this is mint Address of the parent collection
    //             key: Pubkey::from_str("53pYFioA1nhDoLrM8rGJFN4r8J1p1pmHstcchvjEng2h").unwrap(),
    //         }),
    //         uses: None,
    //     },
    //     is_mutable: false,
    //     collection_details: None,
    // };

    // // Create the instruction
    // let instruction = CreateMetadataAccountV3 {
    //     metadata: metadata_pda,
    //     mint: mint_address,
    //     mint_authority: payer.pubkey(),
    //     payer: payer.pubkey(),
    //     update_authority: (payer.pubkey(), true),
    //     system_program: solana_sdk::system_program::ID,
    //     rent: Some(solana_sdk::sysvar::rent::ID),
    // }
    // .instruction(metadata_args);

    // // Get recent blockhash
    // let blockhash = client.get_latest_blockhash()?;
    // let transaction = Transaction::new_signed_with_payer(
    //     &[instruction],
    //     Some(&payer.pubkey()),
    //     &[payer],
    //     blockhash,
    // );

    // // Send and confirm transaction
    // let signature = client.send_and_confirm_transaction(&transaction)?;
    // println!("Metadata account created. Signature: {}", signature);

    // Step 2: Verify the Collection
    let master_edition_pda = find_master_edition_pda(
        &Pubkey::from_str("5iVF9QmA3vCgyyEk7UDJxNRGuFn54Da3mNdnGVrGYgjv").unwrap(),
    );
    println!("Master Edition PDA: {}", master_edition_pda.to_string());
    let set_and_verify_collection = SetAndVerifyCollection {
        metadata: metadata_pda,
        collection_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: payer.pubkey(),
        collection_mint: Pubkey::from_str("5iVF9QmA3vCgyyEk7UDJxNRGuFn54Da3mNdnGVrGYgjv").unwrap(),
        collection: Pubkey::from_str("HMVAGFDBAyBoPH8AwnZH7EtuY7sGBkXtfieuGANVzxBb").unwrap(),
        // fix thies .......
        collection_master_edition_account: master_edition_pda,
        collection_authority_record: None, // Optional: Pass if you have a specific PDA
    };

    // Create the instruction using the struct
    let verify_instruction = set_and_verify_collection.instruction();

    // Create the transaction for verification
    let verify_transaction = Transaction::new_signed_with_payer(
        &[verify_instruction],
        Some(&payer.pubkey()),
        &[payer], // Only payer needs to sign since they are the collection authority
        client.get_latest_blockhash()?,
    );

    let verify_signature = client.send_and_confirm_transaction(&verify_transaction)?;
    println!(
        "Collection verified for NFT. Signature: {}",
        verify_signature
    );

    Ok(())
}

pub fn update(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    // Derive Metadata PDA
    // let metadata_pda = Pubkey::find_program_address(
    //     &[
    //         b"metadata",
    //         mpl_token_metadata::ID.as_ref(),
    //         mint_address.as_ref(),
    //     ],
    //     &mpl_token_metadata::ID,
    // )
    // .0;

    println!("Metadata PDA: {}", metadata_pda);

    // Define Updated Metadata
    let data = DataV2 {
        name: "MetaLoot".to_string(),
        symbol: "LOOT".to_string(),
        uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/metaloot2.json".to_string(),
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Prepare the instruction arguments
    let args = UpdateMetadataAccountV2InstructionArgs {
        data: Some(data),
        new_update_authority: None,  // None if you don't want to change it
        primary_sale_happened: None, // None if no change
        is_mutable: Some(true),      // None if no change
    };

    // Create the instruction using the builder pattern
    let update_instruction = UpdateMetadataAccountV2 {
        metadata: metadata_pda,
        update_authority: payer.pubkey(),
    }
    .instruction(args);

    // Create and send the transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[update_instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Metadata updated successfully. Signature: {}", signature);

    Ok(())
}

pub fn find_master_edition_pda(mint: &Pubkey) -> Pubkey {
    let seeds = &[
        b"metadata",                     // "metadata"
        mpl_token_metadata::ID.as_ref(), // Metaplex Program ID
        mint.as_ref(),                   // Collection Mint Address
        b"edition",                      // "edition"
    ];
    Pubkey::find_program_address(seeds, &mpl_token_metadata::ID).0
}

pub fn update_nfts_collection(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    // // Derive Metadata PDA
    // // let metadata_pda = Pubkey::find_program_address(
    // //     &[
    // //         b"metadata",
    // //         mpl_token_metadata::ID.as_ref(),
    // //         mint_address.as_ref(),
    // //     ],
    // //     &mpl_token_metadata::ID,
    // // )
    // // .0;

    // println!("Metadata PDA: {}", metadata_pda);

    // // Define Updated Metadata
    // let data = DataV2 {
    //     name: "MetaForge Rewards Collection".to_string(),
    //     symbol:  "MFR".to_string(),
    //     uri: "https://tzqzzuafkobkhygtccse.supabase.co/storage/v1/object/public/biz_touch/crypto-ql/Non-FungubleTokenCol.json".to_string(),
    //     seller_fee_basis_points: 0,
    //  creators: Some(vec![Creator {
    //     address: payer.pubkey(),
    //     verified: true,
    //     share: 100, // 100% share
    // }]),
    //     collection: None,
    //     uses: None,
    // };

    // // Prepare the instruction arguments
    // let args = UpdateMetadataAccountV2InstructionArgs {
    //     data: Some(data),
    //     new_update_authority: None,  // None if you don't want to change it
    //     primary_sale_happened: None, // None if no change
    //     is_mutable: Some(true),      // None if no change
    // };

    // // Create the instruction using the builder pattern
    // let update_instruction = UpdateMetadataAccountV2 {
    //     metadata: metadata_pda,
    //     update_authority: payer.pubkey(),
    // }
    // .instruction(args);

    // // Create and send the transaction
    // let recent_blockhash = client.get_latest_blockhash()?;
    // let transaction = Transaction::new_signed_with_payer(
    //     &[update_instruction],
    //     Some(&payer.pubkey()),
    //     &[payer],
    //     recent_blockhash,
    // );

    // let signature = client.send_and_confirm_transaction(&transaction)?;
    // println!("Metadata updated successfully. Signature: {}", signature);

    // Step 2: Create Master Edition
    let create_master_edition_instruction = CreateMasterEditionV3 {
        edition: find_master_edition_pda(&mint_address),
        mint: mint_address,
        update_authority: payer.pubkey(),
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        metadata: metadata_pda,
        token_program: spl_token::ID,
        system_program: solana_sdk::system_program::ID,
        rent: Some(solana_sdk::sysvar::rent::ID),
    }
    .instruction(CreateMasterEditionV3InstructionArgs { max_supply: None }); // Unlimited collection size

    // Send Master Edition Creation Transaction
    let master_edition_tx = Transaction::new_signed_with_payer(
        &[create_master_edition_instruction],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash()?,
    );
    let master_edition_signature = client.send_and_confirm_transaction(&master_edition_tx)?;
    println!(
        "Master Edition account created. Signature: {}",
        master_edition_signature
    );

    Ok(())
}

pub fn get_json_key(
    client: &RpcClient,
    payer: &Keypair,
    mint_address: Pubkey,
    metadata_pda: Pubkey,
) -> Result<(), Box<dyn Error>> {
    // Read keypair from file
    let keypair = Keypair::read_from_file("/Users/copycoder/metaloot-keypair.json")?;
    
    // Get public and private keys
    let public_key = keypair.pubkey();
    let private_key = keypair.secret();

    println!("Public key: {}", public_key);
    println!("Private key2: {:?}", private_key);

    Ok(())
}