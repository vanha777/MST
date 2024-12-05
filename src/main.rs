use mpl_token_metadata::{
    instructions::{UpdateV1, UpdateV1InstructionArgs},
    types::{CollectionDetailsToggle, CollectionToggle, Creator, RuleSetToggle, UsesToggle},
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use std::{io, str::FromStr};
pub mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Choose network:");
    println!("1. Devnet");
    println!("2. Testnet"); 
    println!("3. Mainnet");

    let mut network_choice = String::new();
    io::stdin().read_line(&mut network_choice)?;

    let rpc_url = match network_choice.trim() {
        "1" => "https://api.devnet.solana.com",
        "2" => "https://api.testnet.solana.com",
        "3" => "https://api.mainnet-beta.solana.com",
        _ => {
            println!("Invalid choice, defaulting to devnet");
            "https://api.devnet.solana.com"
        }
    };
    let client = RpcClient::new(rpc_url);

    // Load the payer's keypair
    let payer =
        <solana_sdk::signature::Keypair as solana_sdk::signer::EncodableKey>::read_from_file(
            "/Users/copycoder/metaloot-keypair.json",
        )?;

    println!("Please enter the mint address:");
    let mut mint_address_input = String::new();
    io::stdin().read_line(&mut mint_address_input)?;
    let mint_address = Pubkey::from_str(mint_address_input.trim())?;

    // Derive Metadata PDA
    let metadata_pda = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_address.as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    )
    .0;

    println!("Metadata PDA: {}", metadata_pda);

    println!("Choose operation:");
    println!("1. Create Metadata");
    println!("2. Update Metadata");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => lib::create_metadata(&client, &payer, mint_address, metadata_pda)?,
        "2" => lib::update(&client, &payer, mint_address, metadata_pda)?,
        _ => println!("Invalid choice"),
    }

    Ok(())
}
