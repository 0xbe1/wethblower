use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

#[rustfmt::skip]
mod abi;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let infura_project_id =
        std::env::var("INFURA_PROJECT_ID").expect("INFURA_PROJECT_ID must be set.");
    let provider = Provider::<Ws>::connect(format!(
        "wss://mainnet.infura.io/ws/v3/{}",
        infura_project_id
    ))
        .await?;
    let client = Arc::new(provider);
    let weth_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
        .parse::<Address>()
        .unwrap();
    let weth_contract = abi::weth::WETH::new(weth_address, Arc::clone(&client));

    let events = weth_contract.transfer_filter();
    let mut stream = events.subscribe_with_meta().await?;
    while let Some(Ok((params, meta))) = stream.next().await {
        println!(
            "Tx: {:?}\nAmount: {:?}\n{:?} -> {:?}\n",
            meta.transaction_hash,
            params.wad,
            params.src,
            params.dst
        );
    }

    Ok(())
}
