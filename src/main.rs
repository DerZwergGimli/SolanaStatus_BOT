use std::{env, fs};

use env_logger;
use log::{error, info};
use solanabeach::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let contents = fs::read_to_string("tokenlist.toml")
        .expect("Something went wrong reading the conf file!");

    solana_beach_api::latest_transactions(2).await.unwrap();
    let blocks = solana_beach_api::latest_blocks(50).await.unwrap();


    let timespan = blocks.first().unwrap().block_time.absolute - blocks.last().unwrap().block_time.absolute;

    let mut sum_tx = 0;
    for block in blocks {
        info!("{:?}",block.metrics);
        sum_tx = sum_tx + block.metrics.tx_count;
    }

    info!("TPS={}", sum_tx/timespan);

    println!("Hello, world!");

    Ok(())
}
