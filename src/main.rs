use anyhow::Result;
use solana_client::rpc_client::RpcClient;
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
use instruction::RegistryInstruction;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct RegistryData {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub game_studios: Vec<Pubkey>,
}

fn main() -> Result<()> {
    println!("\x1b[38;5;39mdebug\x1b[0m \x1b[38;5;208m0\x1b[0m");
    // Connect to testnet
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load your keypair
    let payer = read_keypair_file("/Users/copycoder/metaloot-keypair.json").unwrap();
    println!("Using keypair: {}", payer.pubkey());

    // Your program ID (replace with your deployed program ID)
    let program_id = Pubkey::from_str("C4zHMc24dCG2w7cd2inFWMfCT8JY4Si46FZy6F5TFnDV")?;
    println!("\x1b[38;5;39mdebug 2 \x1b[0m \x1b[38;5;208m0\x1b[0m");
    // Sucessfull !
    // Example: Initialize Registry
    // initialize_registry(&client, &payer, &program_id)?;

    // Example: Create Game Studio
    create_game_studio(
        &client,
        &payer,
        &program_id,
        "My Game Studio",
        "MGS",
        "https://example.com/metadata.json",
    )?;

    // Read registry data
    // match get_registry_data(&client, &program_id) {
    //     Ok(data) => println!("\nSuccessfully read registry data: {:?}", data),
    //     Err(e) => println!("Error reading registry data: {}", e),
    // }

    Ok(())
}

fn initialize_registry(client: &RpcClient, payer: &Keypair, program_id: &Pubkey) -> Result<()> {
    println!("\x1b[38;5;39mdebug 3 \x1b[0m \x1b[38;5;208m0\x1b[0m");
    let (registry_pda, _) = Pubkey::find_program_address(&[b"registry"], program_id);

    let instruction = solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::system_program::id(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
        ],
        data: borsh::to_vec(&RegistryInstruction::InitializeRegistry)?,
    };
    println!("\x1b[38;5;39mdebug 4 \x1b[0m \x1b[38;5;208m0\x1b[0m");
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    println!("\x1b[38;5;39mdebug 5 \x1b[0m \x1b[38;5;208m0\x1b[0m");
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Initialize Registry transaction signature: {}", signature);
    Ok(())
}

fn create_game_studio(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    name: &str,
    symbol: &str,
    uri: &str,
) -> Result<()> {
    let (registry_pda, _) = Pubkey::find_program_address(&[b"registry"], program_id);
    let mint = Keypair::new();

    let instruction = solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(mint.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::system_program::id(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::sysvar::rent::id(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: borsh::to_vec(&RegistryInstruction::CreateGameStudio {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
        })?,
    };

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, &mint],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Create Game Studio transaction signature: {}", signature);
    println!("Created mint: {}", mint.pubkey());
    Ok(())
}

fn get_registry_data(client: &RpcClient, program_id: &Pubkey) -> Result<RegistryData> {
    let (registry_pda, _) = Pubkey::find_program_address(&[b"registry"], program_id);
    
    // Fetch the account data
    let account = client.get_account(&registry_pda)?;
    
    // Deserialize the account data using borsh
    let registry_data: RegistryData = borsh::from_slice(&account.data)?;

    println!("\nParsed Registry Data:");
    println!("Initialized: {}", registry_data.is_initialized);
    println!("Admin: {}", registry_data.admin);
    println!("Game Studios (len={}): {:?}", 
        registry_data.game_studios.len(),
        registry_data.game_studios
    );

    Ok(registry_data)
}
