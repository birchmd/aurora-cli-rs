mod client;
mod eth_method;
mod utils;

const AURORA_MAINNET_ENDPOINT: &str = "https://mainnet.aurora.dev";
const NEAR_MAINNET_ENDPOINT: &str = "https://archival-rpc.mainnet.near.org";

use aurora_engine_types::types::{Address, Wei};
use client::{AuroraClient, ClientError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::fs::read_to_string("./.api_key").expect("Failed to read API key; please create a file call .api_key containing you Aurora RPC API key.");
    let aurora_endpoint: String = [AURORA_MAINNET_ENDPOINT, api_key.as_str()].join("/");
    let client = AuroraClient::new(aurora_endpoint, NEAR_MAINNET_ENDPOINT);
    let sk = secp256k1::SecretKey::parse(&[1; 32]).unwrap();
    let source = utils::address_from_secret_key(&sk);
    let target = Address::decode("cc5a584f545b2ca3ebacc1346556d1f5b82b8fc6").unwrap();
    let nonce = client.get_nonce(source).await?;
    let chain_id = client.get_chain_id().await?;
    let tx_hash = client
        .transfer(target, Wei::zero(), &sk, chain_id, nonce)
        .await
        .unwrap();

    println!("AURORA_TX {:?}", tx_hash.aurora_hash);
    println!("NEAR_TX {}", tx_hash.near_hash);
    match client
        .get_transaction_outcome(tx_hash.aurora_hash, "relay.aurora")
        .await
    {
        Err(ClientError::AuroraTransactionNotFound(_)) => {}
        other => panic!("Expected tx not found, but got {:?}", other),
    }

    Ok(())
}
