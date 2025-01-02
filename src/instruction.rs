use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RegistryInstruction {
    InitializeRegistry,
    CreateGameStudio {
        name: String,
        symbol: String,
        uri: String,
    },
    UpdateGameStudio {
        new_uri: Option<String>,
    },
} 