use web3::{Web3, transports::Http, contract::{Contract, Options}};
use web3::types::{H160, U256};
use std::env;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    let infura_project_id = env::var("INFURA_PROJECT_ID")?; // Retrieve the Infura project ID from environment variables
    let http = Http::new(&format!("https://mainnet.infura.io/v3/{}", infura_project_id))?;
    let web3 = Web3::new(http);

    // USDC contract address on Ethereum
    let usdc_address: H160 = "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;

    // USDC contract ABI
    const ABI: &'static str = r#"[
        {
            "constant": true,
            "inputs": [],
            "name": "totalSupply",
            "outputs": [
                {
                    "name": "",
                    "type": "uint256"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    // Create a contract object
    let contract = Contract::from_json(
        web3.eth(),
        usdc_address,
        ABI.as_bytes()
    )?;

    // Call the `totalSupply` method
    let total_supply: U256 = contract.query("totalSupply", (), None, Options::default(), None).await?;

    println!("USDC Total Supply: {}", total_supply);

    Ok(())
}
