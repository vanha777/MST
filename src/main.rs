use anchor_lang::{prelude::{AnchorDeserialize, AnchorSerialize,AccountDeserialize}};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Program,
};
use std::rc::Rc;
use std::str::FromStr;
use shellexpand;

// Define the metadata structure using Anchor's serialization traits
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameRegistryMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
    pub native_token: Option<Pubkey>,
    pub nft_collection: Option<Pubkey>,
}

// Add manual implementation of AccountDeserialize
impl anchor_lang::AccountDeserialize for GameRegistryMetadata {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, anchor_lang::error::Error> {
        Self::try_deserialize(buf)
    }
}

// Add AccountSerialize implementation
impl anchor_lang::AccountSerialize for GameRegistryMetadata {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<(), anchor_lang::error::Error> {
        self.try_to_vec().map(|vec| writer.write_all(&vec))?.map_err(|_| anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::AccountDidNotSerialize))
    }
}

// Define the instruction enum implementing InstructionData
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum MyProgramInstruction {
    Metadata(GameRegistryMetadata),
}

// Add both implementations
impl anchor_lang::InstructionData for MyProgramInstruction {
    fn data(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}

impl anchor_lang::Discriminator for MyProgramInstruction {
    // This is the correct discriminator for "create_game_studio"
    const DISCRIMINATOR: [u8; 8] = [145, 175, 147, 127, 79, 23, 87, 205];

    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

impl anchor_lang::InstructionData for GameRegistryMetadata {
    fn data(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}

impl anchor_lang::Discriminator for GameRegistryMetadata {
    const DISCRIMINATOR: [u8; 8] = [186, 41, 109, 204, 47, 149, 17, 66]; // You'll need to replace this with the correct discriminator

    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // **1. Setup Connection to Local Validator**
    let url = "http://localhost:8899".to_string(); // Although `url` is defined, it's not used directly.

    // **2. Read the Payer's Keypair**
    let payer = read_keypair_file(&*shellexpand::tilde("~/metaloot-keypair.json"))?;

    // **3. Initialize the Anchor Client**
    let client = Client::new_with_options(
        anchor_client::Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );

    // **4. Specify Your Program ID**
    let program_id = Pubkey::from_str("v3MbKaZSQJrwZWUz81cQ3kc8XvMsiNNxZjM3vN5BB32")?;
    let program = client.program(program_id)?;

    // **5. Generate Required Keypairs**
    let entry_seed = read_keypair_file(&*shellexpand::tilde("~/metaloot-keypair.json"))?;       // This is the only keypair we need

    // **6. Derive PDA (Program Derived Address)**
    let binding = entry_seed.pubkey();
    let seeds = &[b"registry", binding.as_ref()];
    let (pda, _bump) = Pubkey::find_program_address(seeds, &program_id);

    // **7. Prepare Metadata**
    let metadata = MyProgramInstruction::Metadata(GameRegistryMetadata {
        name: "Awesome Game Studio".to_string(),
        symbol: "AGS".to_string(),
        uri: "https://awesome-games.com/metadata.json".to_string(),
        creator: read_keypair_file(&*shellexpand::tilde("~/metaloot-keypair.json"))?.pubkey(),
        native_token: None,  // Changed to None since these are optional
        nft_collection: None,
    });

    println!("Creating game studio...");

    // **8. Create Game Studio via Instruction**
    let signature = program
        .request()
        .accounts(create_game_studio_accounts(
            &read_keypair_file(&*shellexpand::tilde("~/metaloot-keypair.json"))?.pubkey(),
            &pda,
            &entry_seed.pubkey(),
        ))
        .args(metadata)
        .signer(&read_keypair_file(&*shellexpand::tilde("~/metaloot-keypair.json"))?)
        .signer(&entry_seed)  // Only need entry_seed signer
        .send()
        .await?;

    println!("Transaction signature: {}", signature);

    // **9. Fetch and Display the Created Account**
    let account: GameRegistryMetadata = program.account(pda).await?;

    println!("\nCreated Game Studio:");
    println!("Name: {}", account.name);
    println!("Symbol: {}", account.symbol);
    println!("URI: {}", account.uri);
    println!("Creator: {}", account.creator);
    println!("Native Token: {:?}", account.native_token);
    println!("NFT Collection: {:?}", account.nft_collection);

    Ok(())
}

// **10. Simplified Accounts Structure**
fn create_game_studio_accounts(
    payer: &Pubkey,
    pda: &Pubkey,
    entry_seed: &Pubkey,
) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
    use anchor_lang::solana_program::instruction::AccountMeta;
    vec![
        AccountMeta::new(*payer, true),                // payer: Signer<'info>
        AccountMeta::new(*pda, false),                // pda: Account<'info, GameRegistryMetadata>
        AccountMeta::new_readonly(*entry_seed, false), // entry_seed: AccountInfo<'info>
        AccountMeta::new_readonly(system_program::ID, false), // system_program: Program<'info, System>
    ]
}
