use crate::utils::env_var::get_env_var;
use crate::utils::schema::Block;
use crate::utils::schema::Network;
use ethers::types::H256;
use ethers::utils::hex;
use ethers::{prelude::*, utils};
use ethers_providers::{Http, Provider};
use std::str::FromStr;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn send_wvm_calldata(block_data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let network = Network::config();
    let provider = Network::provider(&network, true).await;
    let private_key = get_env_var("archiver_pk").unwrap();
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(network.wvm_chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let address_from = network.archiver_address.parse::<Address>()?;
    let address_to = network.archive_pool_address.parse::<Address>()?;
    // check archiver tWVM balance (non-zero)
    assert_non_zero_balance(&provider, &address_from).await;
    // send calldata tx to WeaveVM
    let txid = send_transaction(&client, &address_from, &address_to, block_data).await?;

    Ok(txid)
}

async fn assert_non_zero_balance(provider: &Provider<Http>, address: &Address) {
    let balance = provider.get_balance(address.clone(), None).await.unwrap();
    assert!(balance > 0.into());
}

pub async fn get_archiver_balance() -> U256 {
    let network = Network::config();
    let provider = Network::provider(&network, true).await;
    let address = network.archiver_address.parse::<Address>().unwrap();
    let balance = provider.get_balance(address, None).await.unwrap();
    balance
}

async fn send_transaction(
    client: &Client,
    address_from: &Address,
    address_to: &Address,
    block_data: Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "\nArchiving block data from archiver: {} to archive pool: {}",
        address_from, address_to
    );
    // 7wei equivalent to 0.000000007Gwei
    let gas_price = U256::from(7);
    let tx = TransactionRequest::new()
        .to(address_to.clone())
        .value(U256::from(utils::parse_ether(0)?))
        .from(address_from.clone())
        .data(block_data)
        .gas_price(gas_price);

    let tx = client.send_transaction(tx, None).await?.await?;
    let json_tx = serde_json::json!(tx);
    let txid = json_tx["transactionHash"].to_string();

    println!("\nWeaveVM Archiving TXID: {}", txid);
    Ok(txid)
}

pub async fn decode_wvm_tx_data(txid: &str) -> Block {
    let network = Network::config();
    let provider = network.provider(true).await;
    let txid = H256::from_str(&txid).unwrap();
    let tx = provider.get_transaction(txid).await.unwrap();

    let tx_json = serde_json::json!(&tx);
    let tx_input_raw = tx_json["input"].as_str().unwrap_or("0x");
    let byte_array = hex::decode(tx_input_raw.trim_start_matches("0x")).expect("decoding failed");

    let brotli_decompressed = Block::brotli_decompress(byte_array);
    let borsh_derserialized = Block::borsh_der(brotli_decompressed);
    println!("{:?}", borsh_derserialized);
    borsh_derserialized
}
