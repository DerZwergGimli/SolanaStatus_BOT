use std::{env, fs};
use std::collections::HashSet;

use env_logger;
use log::{error, info};
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use solanabeach::*;

#[tokio::main]
async fn main() {
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

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
