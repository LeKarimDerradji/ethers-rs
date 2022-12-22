use ethers::{
    core::utils::format_ether,
    prelude::*,
    providers::{Http, Provider},
};


use crate::provider::providermod;

mod provider;

#[tokio::main]
async fn main() {
    // Get the balance of an Ethereum address
    let address_slice =
        b"\xd8\xdA\x6B\xF2\x69\x64\xaf\x9D\x7e\xEd\x9e\x03\xE5\x34\x15\xD3\x7a\xA9\x60\x45";
    let address = Address::from_slice(address_slice);

    let balance = providermod::get_balance_mod(address);

    match balance.await {
        Ok(balance) => println!("Balance: {}", format_ether(balance)),
        Err(error) => println!("Error: {}", error),
    }

    let block_number_future = providermod::get_block_number();
    //let block_number = block_number_future.await;
    //println!("Current block number: {}", block_number);
}
