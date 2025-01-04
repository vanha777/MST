use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RegistryInstruction {
    InitializeRegistry {},
    CreateGameStudio {
        name: String,
        symbol: String,
        uri: String,
        creator: Pubkey,
    },
    UpdateGameStudio {
        new_uri: Option<String>,
    },
}