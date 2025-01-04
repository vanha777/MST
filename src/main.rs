use anyhow::Result;
use solana_client::{
    rpc_client::RpcClient,
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
mod instruction;
use borsh::{BorshDeserialize, BorshSerialize};
use spl_token;
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameRegistryMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RegistryData {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub game_studios: Vec<Pubkey>,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateGameRegistryMetadata {
    pub name: Option<String>,   // Game name
    pub symbol: Option<String>, // Game symbol or short identifier
    pub uri: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RegistryInstruction {
    /// Initialize the registry account (and set admin).
    // InitializeRegistry {
    //     // No additional fields needed; admin is the signer
    // },

    /// Create a new game studio NFT and register it.
    CreateGameStudio(GameRegistryMetadata),

    // /// Update an existing game studio NFT's metadata.
    UpdateGameStudio(GameRegistryMetadata),
}

fn main() -> Result<()> {
    println!("\x1b[1;36mWelcome to the Solana Registry Client!\x1b[0m");
    println!("\x1b[1;33mAvailable commands:\x1b[0m");
    println!("\x1b[1;32m  create-studio   - Create a new game studio (requires: token_mint, name, symbol, uri)\x1b[0m");
    println!("\x1b[1;32m  update-studio   - Update a game studio (requires: token_mint, name, symbol, uri)\x1b[0m");
    println!("\x1b[1;32m  read            - Read registry data\x1b[0m");
    println!("\x1b[1;32m  list-studios    - List all game studios\x1b[0m");
    println!("\n\x1b[1;35mPlease enter a command:\x1b[0m");

    // Get user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let args: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

    // Connect to testnet
    let rpc_url = "http://127.0.0.1:8899".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load keypair
    let payer = read_keypair_file("/Users/copycoder/metaloot-keypair.json").unwrap();
    println!("Using keypair: {}", payer.pubkey());

    // Program ID
    let program_id = Pubkey::from_str("Dde56tSF9q1iepWVoFg2HEGPEWhVPpgsYqKztUEU69n1")?;

    // Parse command
    if args.is_empty() {
        println!("No command provided. Please try again.");
        return Ok(());
    }

    match args[0].as_str() {
        "create-studio" => {
            if args.len() < 5 {
                println!("Please enter the following details for the game studio:");
                println!("Token Mint (Pubkey): ");
                let mut token_mint = String::new();
                std::io::stdin().read_line(&mut token_mint)?;
                let token_mint = Pubkey::from_str(token_mint.trim())?;

                println!("Name: ");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name)?;

                println!("Symbol: ");
                let mut symbol = String::new();
                std::io::stdin().read_line(&mut symbol)?;

                println!("URI: ");
                let mut uri = String::new();
                std::io::stdin().read_line(&mut uri)?;

                println!("Creating game studio...");
                create_game_studio(
                    &client,
                    &payer,
                    &program_id,
                    &token_mint,
                    name.trim(),
                    symbol.trim(),
                    uri.trim(),
                )?;
            } else {
                let token_mint = Pubkey::from_str(&args[1])?;
                create_game_studio(
                    &client,
                    &payer,
                    &program_id,
                    &token_mint,
                    &args[2],
                    &args[3],
                    &args[4],
                )?;
            }
        }
        "update-studio" => {
            if args.len() < 5 {
                println!("Please enter the following details to update the game studio:");
                println!("Token Mint (Pubkey): ");
                let mut token_mint = String::new();
                std::io::stdin().read_line(&mut token_mint)?;
                let token_mint = Pubkey::from_str(token_mint.trim())?;

                println!("New Name (or press enter to skip): ");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name)?;
                let name = if name.trim().is_empty() {
                    None
                } else {
                    Some(name.trim().to_string())
                };

                println!("New Symbol (or press enter to skip): ");
                let mut symbol = String::new();
                std::io::stdin().read_line(&mut symbol)?;
                let symbol = if symbol.trim().is_empty() {
                    None
                } else {
                    Some(symbol.trim().to_string())
                };

                println!("New URI (or press enter to skip): ");
                let mut uri = String::new();
                std::io::stdin().read_line(&mut uri)?;
                let uri = if uri.trim().is_empty() {
                    None
                } else {
                    Some(uri.trim().to_string())
                };

                println!("Updating game studio...");
                update_game_studio(&client, &payer, &program_id, &token_mint, name, symbol, uri)?;
            } else {
                let token_mint = Pubkey::from_str(&args[1])?;
                update_game_studio(
                    &client,
                    &payer,
                    &program_id,
                    &token_mint,
                    Some(args[2].clone()),
                    Some(args[3].clone()),
                    Some(args[4].clone()),
                )?;
            }
        }
        "list-studios" => {
            println!("Listing all game studios...");
            match get_all_game_studios(&client, &program_id) {
                Ok(studios) => {
                    for studio in studios {
                        println!("Game Studio: {:?}", studio);
                    }
                }
                Err(e) => println!("Error listing game studios: {}", e),
            }
        }
        _ => {
            println!("Unknown command. Available commands: create-studio, update-studio, read, list-studios");
        }
    }

    Ok(())
}

fn create_game_studio(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    token_mint: &Pubkey,
    name: &str,
    symbol: &str,
    uri: &str,
) -> Result<()> {
    println!("\x1b[1;36mCreating game studio with:\x1b[0m");
    println!("\x1b[1;33mName: {}\x1b[0m", name);
    println!("\x1b[1;33mSymbol: {}\x1b[0m", symbol);
    println!("\x1b[1;33mURI: {}\x1b[0m", uri);

    // First verify registry exists and is initialized
    // let (registry_pda, _) = Pubkey::find_program_address(&[b"registry"], program_id);
    // println!("\x1b[1;35mRegistry PDA: {}\x1b[0m", registry_pda);

    // Verify registry is initialized
    // match client.get_account(&registry_pda) {
    //     Ok(account) => {
    //         let registry_data: RegistryData = borsh::from_slice(&account.data)?;
    //         println!("\x1b[1;34mRegistry initialized: {}\x1b[0m", registry_data.is_initialized);
    //         println!("\x1b[1;34mRegistry admin: {}\x1b[0m", registry_data.admin);
    //     }
    //     Err(e) => {
    //         println!("\x1b[1;31mFailed to fetch registry account: {}\x1b[0m", e);
    //         return Err(anyhow::anyhow!("Registry account not found"));
    //     }
    // }

    let entry_seeds = &[b"registry", token_mint.as_ref()];
    let (entry_pda, bump) = Pubkey::find_program_address(entry_seeds, program_id);
    println!("\x1b[1;35mEntry PDA: {} (bump: {})\x1b[0m", entry_pda, bump);

    // Create instruction data
    let instruction_data = RegistryInstruction::CreateGameStudio(GameRegistryMetadata {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        creator: payer.pubkey(),
    });
    let serialized_instruction = borsh::to_vec(&instruction_data)?;
    println!(
        "\x1b[1;34mInstruction bytes: {:?}\x1b[0m",
        serialized_instruction
    );

    let instruction = solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            // solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::system_program::id(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new(entry_pda, false),
            solana_sdk::instruction::AccountMeta::new_readonly(*token_mint, false),
        ],
        data: serialized_instruction,
    };

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    println!("\x1b[1;32mSending transaction...\x1b[0m");
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!(
                "\x1b[1;32mSuccess! Transaction signature: {}\x1b[0m",
                signature
            );
            Ok(())
        }
        Err(e) => {
            println!("\x1b[1;31mTransaction failed: {}\x1b[0m", e);
            println!("\x1b[1;31mError details: {:?}\x1b[0m", e);
            Err(anyhow::anyhow!(e))
        }
    }
}

fn update_game_studio(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    token_mint: &Pubkey,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
) -> Result<()> {
    println!("\x1b[1;36mUpdating game studio with:\x1b[0m");
    if let Some(name) = &name {
        println!("\x1b[1;33mNew Name: {}\x1b[0m", name);
    }
    if let Some(symbol) = &symbol {
        println!("\x1b[1;33mNew Symbol: {}\x1b[0m", symbol);
    }
    if let Some(uri) = &uri {
        println!("\x1b[1;33mNew URI: {}\x1b[0m", uri);
    }

    let entry_seeds = &[b"registry", token_mint.as_ref()];
    let (entry_pda, bump) = Pubkey::find_program_address(entry_seeds, program_id);
    println!("\x1b[1;35mEntry PDA: {} (bump: {})\x1b[0m", entry_pda, bump);

    // Create instruction data
    let instruction_data = RegistryInstruction::UpdateGameStudio(GameRegistryMetadata {
        name: name.unwrap_or_default(),
        symbol: symbol.unwrap_or_default(), 
        uri: uri.unwrap_or_default(),
        creator: payer.pubkey(),
    });
    let serialized_instruction = borsh::to_vec(&instruction_data)?;
    println!(
        "\x1b[1;34mInstruction bytes: {:?}\x1b[0m",
        serialized_instruction
    );

    let instruction = solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(entry_pda, false),
            solana_sdk::instruction::AccountMeta::new_readonly(*token_mint, false),
        ],
        data: serialized_instruction,
    };

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    println!("\x1b[1;32mSending transaction...\x1b[0m");
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!(
                "\x1b[1;32mSuccess! Transaction signature: {}\x1b[0m",
                signature
            );
            Ok(())
        }
        Err(e) => {
            println!("\x1b[1;31mTransaction failed: {}\x1b[0m", e);
            println!("\x1b[1;31mError details: {:?}\x1b[0m", e);
            Err(anyhow::anyhow!(e))
        }
    }
}

fn get_all_game_studios(
    client: &RpcClient,
    program_id: &Pubkey,
) -> Result<Vec<GameRegistryMetadata>> {
    println!("\x1b[1;32mFetching game studios...\x1b[0m");

    // Get all program accounts without filtering
    let accounts = client.get_program_accounts(program_id)?;
    println!("\x1b[1;32mFound {} total accounts\x1b[0m", accounts.len());

    let game_studios = accounts
        .iter()
        .filter_map(|(pubkey, account)| {
            // Try to deserialize as GameRegistryMetadata
            match GameRegistryMetadata::try_from_slice(&account.data) {
                Ok(metadata) => {
                    println!(
                        "\x1b[1;34mFound game studio: {} at {}\x1b[0m",
                        metadata.name, pubkey
                    );
                    Some(metadata)
                }
                Err(_) => {
                    // Silently skip accounts that don't match our expected format
                    None
                }
            }
        })
        .collect();

    Ok(game_studios)
}
