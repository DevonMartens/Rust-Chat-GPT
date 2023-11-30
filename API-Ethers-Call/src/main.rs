use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::io::{self, Write};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;


// Type alias for the client which combines a provider and a wallet for signing transactions.
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>; 
// Function to calculate the Uniswap V2 fee (assuming a fixed fee rate of 3%).
fn calculate_uniswap_v2_fee(amount: f64) -> f64 {
    0.03 * amount
}

// Asynchronous main function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // Load environment variables from .env file

    let infura_project_id = env::var("INFURA_PROJECT_ID")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let wallet_address = env::var("WALLET_ADDRESS")?;
    // Create a provider to interact with the Ethereum network.
    // Replace the URL with the appropriate endpoint in production.
    let provider = Provider::<Http>::try_from(format!("https://goerli.infura.io/v3/{}", infura_project_id))?;
    let chain_id = provider.get_chainid().await?; // Retrieve the chain ID.

    // Create a wallet from a private key string.
    // WARNING: Do not hardcode private keys in production. Use environment variables or a secure key management system.
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());


    // The address of the Uniswap V2 router contract.
    let contract_address = Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D")?;

    // Uniswap V2 router contract ABI.
    // The ABI from the router contract so that the script has the needed data to execute the fuction.
    const ABI: &'static str = r#"[
        {
            "constant": false,
            "inputs": [
                {
                    "name": "amountIn",
                    "type": "uint256"
                },
                {
                    "name": "amountOutMin",
                    "type": "uint256"
                },
                {
                    "name": "path",
                    "type": "address[]"
                },
                {
                    "name": "to",
                    "type": "address"
                },
                {
                    "name": "deadline",
                    "type": "uint256"
                }
            ],
            "name": "swapExactTokensForTokens",
            "outputs": [
                {
                    "name": "amounts",
                    "type": "uint256[]"
                }
            ],
            "payable": false,
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;

    let parsed_abi: Abi = serde_json::from_str(ABI)?;

    // Create a client by combining the provider and wallet, enabling transaction signing.
    let client = SignerMiddleware::new(provider, wallet);

    // Initialize the Uniswap V2 router contract instance for function calls.
    let contract = Contract::new(contract_address, parsed_abi, client.into());

    // User input section for the swap parameters.
    // ... (User input code for token addresses, amounts, and other parameters) ...
    // Capture token addresses and amounts from user input
    // Token being sold - eth address
    print!("Enter input token address: ");
    io::stdout().flush().unwrap();
    // Input is users data back will like be a variable grabbed in arbitrage
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let token1_address: Address = input.trim().parse().unwrap();

    // Token being bought - eth address
    print!("Enter output token address: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let token2_address: Address = input.trim().parse().unwrap();

    // This is the quantity of the token you are willing to swap or trade.
    print!("Enter input amount: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let amount_in: U256 = U256::from_dec_str(input.trim()).unwrap();

    // Calculate and display the Uniswap fee.
    let amount_in_f64: f64 = amount_in.low_u64() as f64; // Convert U256 to f64 for fee calculation.
    let fee = calculate_uniswap_v2_fee(amount_in_f64);
    println!("The Uniswap V2 fee will be: {}", fee);

    // When swapping tokens, there are potential price fluctuations and other factors that
    // might change the amount of the output token you receive. The "min output"
    // is a user-defined threshold that specifies the minimum amount of the output token they are willing to accept for the swap.
    print!("Enter minimum output amount: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let amount_out_min: U256 = U256::from_dec_str(input.trim()).unwrap();

    // Specifies pool
    let path: Vec<Address> = vec![token1_address, token2_address];

    // Address of sending wallet - we may want this to be adjustable in prod
    let to: Address = wallet_address.parse()?;

    // The deadline ensures that the transaction only goes through if it's processed before the specified time.
    let deadline: U256 = U256::from_dec_str("9999999999999999").expect("Failed to parse deadline");

    // Calls
    let call = contract.method::<_, bool>(
        "swapExactTokensForTokens",
        (amount_in, amount_out_min, path, to, deadline),
    )?;

    let gas_estimate = call.estimate_gas().await?;
    println!("Estimated gas: {:?}", gas_estimate);

    //input token
    let amount_in_f64: f64 = amount_in.low_u64() as f64;
    let fee = calculate_uniswap_v2_fee(amount_in_f64);
    println!("The Uniswap V2 fee will be: {}", fee);

    // Prompt user to proceed or not
    print!("Do you want to execute the transaction? (yes/no): ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().to_lowercase();
    if choice == "yes" {
        // Prints transaction reciept
        // We will need to record this in product and ensure it executes
        let tx_receipt = call.send().await?;
        println!("Transaction sent: {:?}", tx_receipt);
    }
    // Tell you it's canceled
    else {
        println!("Transaction canceled.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_uniswap_v2_fee() {
        let amount = 100.0;
        let fee = calculate_uniswap_v2_fee(amount);
        assert_eq!(fee, 3.0);
    }
}