use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs, UpdateV1,UpdateMetadataAccountV2,UpdateMetadataAccountV2InstructionArgs},
    types::{
        CollectionDetailsToggle, CollectionToggle, Creator, Data, DataV2, RuleSetToggle, UsesToggle,
    },
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
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
        new_update_authority: None,      // None if you don't want to change it
        primary_sale_happened: None,     // None if no change
        is_mutable: Some(true),                // None if no change
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

