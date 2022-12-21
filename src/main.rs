use ethers::{
    core::utils::format_ether,
    prelude::*,
    providers::{Http, Provider},
};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Get the value of the "INFURA_API_KEY" environment variable
    let infura_api_key = std::env::var("INFURA_API_KEY").unwrap_or_else(|_| "default value".to_string());
    println!("INFURA_API_KEY = {}", infura_api_key);

    // Concatenate the API key to the API string
    let api_string = format!("https://mainnet.infura.io/v3/{}", infura_api_key);

    /*
    Create a provider for connecting to the Ethereum network
    */
    let provider = Provider::<Http>::try_from(api_string.as_str()).unwrap();

    // Get the balance of an Ethereum address
    let address_slice =
        b"\xd8\xdA\x6B\xF2\x69\x64\xaf\x9D\x7e\xEd\x9e\x03\xE5\x34\x15\xD3\x7a\xA9\x60\x45";
    let address = Address::from_slice(address_slice);

    let balance_future = provider.get_balance(address, None);
    let balance = balance_future.await;

    match balance {
        Ok(balance) => println!("Balance: {}", format_ether(balance)),
        Err(error) => println!("Error: {}", error),
    }

    let block_number_future = provider.get_block_number();
    let block_number = block_number_future.await.unwrap();
    println!("Current block number: {}", block_number);
}
