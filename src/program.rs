use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction as token_instruction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], // e.g. amount to transfer could be here
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Let's say we expect:
    // 1. The admin (signer)
    // 2. The "escrow" token account owned by PDA
    // 3. The user's token account to receive tokens
    // 4. The token program
    // 5. The system program (if needed)
    // We assume these accounts are provided in the correct order.

    let admin_ai = next_account_info(account_info_iter)?;
    let escrow_ai = next_account_info(account_info_iter)?;
    let user_token_ai = next_account_info(account_info_iter)?;
    let token_program_ai = next_account_info(account_info_iter)?;

    // Parse the amount from instruction_data
    // For simplicity, assume instruction_data is just 8 bytes representing a u64 amount.
    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Derive PDA (program derived address)
    let seed_str = b"state";
    // Typically you have a known bump. Let's say we've precomputed or stored it somewhere.
    // For demonstration, we'll hardcode a bump. In practice, you’d find a bump by trying
    // create_program_address until it succeeds.
    let bump: u8 = 255; // A placeholder. Real code must find a bump that works.

    // Derive the PDA using the seeds:
    let (pda, derived_bump) = Pubkey::find_program_address(&[seed_str], program_id);
    if derived_bump != bump {
        msg!("Bump does not match the expected bump");
        return Err(ProgramError::InvalidSeeds);
    }

    // Check if the derived pda matches what we expect to control the escrow account.
    // The escrow should have its owner set to pda.
    // (This check might be optional if you trust your input, but it's good practice.)
    // You would also verify that escrow_ai.owner == token_program, and escrow_ai.key == expected token account.

    // Check admin authorization: The admin must be a signer.
    if !admin_ai.is_signer {
        msg!("Admin must sign the transaction.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Now we want to transfer tokens from escrow to user using SPL token program.
    // The SPL token transfer instruction requires:
    //   token_program id
    //   source account (escrow_ai)
    //   destination account (user_token_ai)
    //   authority (the PDA)
    //   and amount.

    let transfer_ix = token_instruction::transfer(
        token_program_ai.key,
        escrow_ai.key,
        user_token_ai.key,
        &pda, // authority is the PDA
        &[],  // no additional signers
        amount,
    )?;

    // Because the PDA does not have a private key, we must call invoke_signed with the PDA seeds.
    // invoke_signed takes an array of seed slices. The PDA is derived by these seeds.
    // We use the same seeds: [b"state", [bump]].
    let signer_seeds: &[&[u8]] = &[seed_str, &[bump]];

    // Execute the transfer via CPI
    invoke_signed(
        &transfer_ix,
        &[
            escrow_ai.clone(),
            user_token_ai.clone(),
            token_program_ai.clone(),
        ],
        &[signer_seeds], // Sign with the PDA
    )?;

    msg!("Token transfer succeeded from escrow PDA to user");

    Ok(())
}
