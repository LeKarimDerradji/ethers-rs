// Define a `provider` module to manage the provider and its functions
pub(crate) mod providermod {
    use ethers::{
        prelude::*,
        providers::{Http, Provider},
    };
    use std::{env, error::Error};

    // Define a private function to create a provider
    fn create_provider() -> Provider<Http> {
        // Get the value of the "INFURA_API_KEY" environment variable
        let infura_api_key = env::var("INFURA_API_KEY").unwrap_or("default value".to_string());

        // Concatenate the API key to the API string
        let api_string = format!("https://mainnet.infura.io/v3/{}", infura_api_key);

        // Create a provider using the `Http` type from the `ethers` crate and the connection URL
        Provider::<Http>::try_from(api_string.as_str()).unwrap()
    }

    // Define a public function to retrieve the balance of an Ethereum address
    pub async fn get_balance_mod(address: Address) -> Result<U256, Box<dyn Error>> {
        // Create a provider
        let provider = create_provider();

        // Call the `get_balance` function on the provider with the address as an argument
        let balance_future = provider.get_balance(address, None);
        let balance = balance_future.await?;

        Ok(balance)
    }

    // Define a public function to retrieve the current block number
    pub async fn get_block_number() -> Result<u64, Box<dyn Error>> {
        // Create a provider
        let provider = create_provider();

        // Call the `get_block_number` function on the provider
        let block_number_future = provider.get_block_number();
        let block_number = block_number_future.await?;

        Ok(block_number)
    }
}
