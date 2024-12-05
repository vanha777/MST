# Step-by-Step Guide to Create and Manage a Solana Token

## 1. Create a Keypair
1. Generate a new keypair:
   - Run `solana-keygen new --outfile ~/my-token-keypair.json`
   - Securely save the displayed seed phrase
2. View your public key:
   - Run `solana-keygen pubkey ~/my-token-keypair.json`

## 2. Set up Solana CLI
1. Configure network:
   - For devnet: `solana config set --url https://api.devnet.solana.com`
   - For mainnet: `solana config set --url https://api.mainnet-beta.solana.com`
2. Verify configuration with `solana config get`

## 3. Fund your wallet 
1. For devnet:
   - Request airdrop: `solana airdrop 2 <YOUR-WALLET-ADDRESS>`
2. For mainnet:
   - Transfer real SOL to your wallet
3. Check balance: `solana balance <YOUR-WALLET-ADDRESS>`

## 4. Create Token
1. Create token using SPL Token Program:
   - Run `spl-token create-token` with desired decimals
   - Save the generated token address
2. Default decimals is 9 (like SOL)
Ex: `spl-token mint  <MINT-ADDRESS> <AMOUNT> --mint-authority <PAIR-KEY-PATH>`

## 5. Create Account Holder for the Token
1. Create account for holding tokens:
   - Run `spl-token create-account <TOKEN-ADDRESS>`
2. This account will store your initial token supply
Ex: `spl-token create-account <TOKEN-ADDRESS>`

## 6. Create Metadata Account (Optional)
1. Prepare metadata:
   - Create a metadata.json file with token details:
     ```json
     {
       "name": "Your Token Name",
       "symbol": "SYMBOL", 
       "description": "Token description",
       "image": "https://your-image-url.com/image.png",
       "attributes": [],
       "properties": {
         "category": "token",
         "creators": [
           {
             "address": "<YOUR-WALLET-ADDRESS>",
             "share": 100
           }
         ]
       }
     }
     ```
   - Upload metadata.json and image to a public URL

2. Run the metadata program:
   ```bash
   # Navigate to program directory
   cd your-program-directory
   
   # Build and run
   cargo run
   ```

3. Follow the prompts:
   - Choose network (1=Devnet, 2=Testnet, 3=Mainnet)
   - Enter your token's mint address
   - Choose operation:
     - 1: Create new metadata
     - 2: Update existing metadata

4. Verify metadata:
   - Check transaction signature
   - View on Solana Explorer
   - Metadata should be linked to your token


## 7. Mint Tokens
1. Mint initial token supply:
   - Use `spl-token mint` command
   - Specify amount and destination
2. Only mint authority can create new tokens
Ex: `spl-token mint  <MINT-ADDRESS> <AMOUNT> --mint-authority <PAIR-KEY-PATH>`

## 8. Create Account Holder for the Receiver
1. Create token account for recipient:
   - Each receiver needs their own token account
   - Account must be associated with the token's mint address
Ex: `spl-token create-account <TOKEN-ADDRESS> --owner <RECEIVER-ADDRESS> --fee-payer <YOUR-KEY-PATH>`

## 9. Transfer Tokens
1. Send tokens to other wallets:
   - Ensure recipient has a token account
   - Use `spl-token transfer` command
2. Keep track of transaction signatures
Ex: `spl-token transfer <TOKEN-ADDRESS> <AMOUNT> <RECEIVER-ADDRESS> --fee-payer <YOUR-KEY-PATH>`

## 10. Remove Mint Authority (Optional)
1. Disable minting to fix total supply:
   - This action is irreversible
   - Makes token supply immutable
Ex: `spl-token authorize <TOKEN-ADDRESS> mint --disable`

### Important Notes:
- Always backup keypair and seed phrase
- Test thoroughly on devnet first
- Maintain sufficient SOL for transaction fees
- Document all addresses and transactions
- Use hardware wallet for production tokens
- Verify transactions on Solana Explorer

### Common Commands:
- Check balance: `spl-token balance <TOKEN-ADDRESS>`
- View supply: `spl-token supply <TOKEN-ADDRESS>`
- List tokens: `spl-token accounts`
- Close empty account: `spl-token close <TOKEN-ADDRESS>`