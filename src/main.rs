use crate::bot_helper::bot_start;

mod bot_helper;
mod commands;
mod tps_calculator;

#[tokio::main]
async fn main() {
    println!("Starting Application...");
    env_logger::init();

    bot_start().await;
}
