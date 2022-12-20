use ethers::{
    contract::{Contract, ContractFactory},
    core::utils::format_ether,
    prelude::*,
    providers::{Http, Provider},
};

use tokio;

#[tokio::main]
async fn main() {
    // Create a provider for connecting to the Ethereum network
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/bfecac101c384372945929d0ca759859")
            .unwrap();

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