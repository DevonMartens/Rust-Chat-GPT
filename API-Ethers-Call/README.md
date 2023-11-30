# Uniswap V2 Router Interaction Script

## Description
This Rust script interacts with the Uniswap V2 router contract on the Ethereum blockchain. It enables users to swap tokens by calling the `swapExactTokensForTokens` function of the Uniswap V2 router.

## Features
- Connect to Ethereum blockchain using ethers-rs.
- Perform token swaps on Uniswap V2.
- Calculate transaction fees.
- Estimate gas for the transaction.
- User confirmation before sending the transaction.

## Requirements
- Rust programming environment.
- ethers-rs library.

## Usage
1. **Set up the Rust environment**: Ensure you have Rust installed.
2. **Add ethers-rs dependency**: Include `ethers` in your `Cargo.toml`.
3. **Configure the script**: Replace placeholders such as `YOUR_INFURA_PROJECT_ID` and `YOUR_PRIVATE_KEY` with your actual Infura project ID and private key.
4. **Run the script**: Use `cargo run` to execute the script.
5. **Follow on-screen prompts**: Enter the required token swap parameters when prompted.


## .env

    ```
    INFURA_PROJECT_ID=your_infura_project_id
    PRIVATE_KEY=your_private_key
    WALLET_ADDRESS=your_wallet_address
    ```