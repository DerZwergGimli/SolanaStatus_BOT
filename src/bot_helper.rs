use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use log::{error, info, warn};
use serenity::async_trait;
use serenity::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::RoleId;
use serenity::prelude::*;
use serenity::prelude::GatewayIntents;
use solscan_api::solscan::SolscanAPI;

use crate::commands::ping::*;
use crate::tps_calculator::calculate_tps;

#[group]
#[commands(ping)]
struct General;

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache built successfully!");
        let ctx = Arc::new(ctx);
        let solscan_api = SolscanAPI::new();
        let tps_threshold = env::var("TPS_THRESHOLD").unwrap_or_else(|_| "1000".to_string()).parse::<i64>().unwrap();

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    warn!("Running UpdateName LOOP");

                    //Fetch TPS
                    let mut tps = 0;
                    let mut tps_string: String = "".to_string();
                    match solscan_api.get_block_last(Some(20)).await {
                        Ok(blocks) => {
                            tps = calculate_tps(blocks);
                            println!("tps: {}", tps);
                            if tps > 0 {
                                if tps > tps_threshold {
                                    tps_string = format!("🚀 ~{} TPS", tps)
                                } else {
                                    tps_string = format!("🔥 ~{} TPS", tps)
                                }
                            }
                        }
                        Err(err) => {
                            error!("Error fetching solscanAPI: {:?}", err)
                        }
                    }

                    //Change Nickname
                    if !tps_string.is_empty() {
                        for _guild in _guilds.clone() {
                            match _guild.edit_nickname(&ctx1.http, Some(tps_string.as_str())).await {
                                Ok(_) => { info!("Changed Bot nickname!") }
                                Err(_) => { error!("Unable to change bot nickname!") }
                            };
                        }
                    }

                    //Change Bot-Color
                    let mut red_role_id: RoleId = Default::default();
                    let mut green_role_id: RoleId = Default::default();
                    for guild in _guilds.clone() {
                        let roles = guild.roles(&ctx).await;
                        for (_role_id, role) in roles.expect("no guild roles found!") {
                            if role.name.contains("tickers-red") {
                                red_role_id = role.id;
                            }
                            if role.name.contains("tickers-green") {
                                green_role_id = role.id;
                            }
                        }
                    }


                    for guild in &_guilds {
                        if tps > tps_threshold {
                            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().remove_role(&ctx, red_role_id).await.unwrap();
                            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().add_role(&ctx, green_role_id).await.unwrap();
                        } else {
                            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().remove_role(&ctx, green_role_id).await.unwrap();
                            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().add_role(&ctx, red_role_id).await.unwrap();
                        }
                    }


                    tokio::time::sleep(Duration::from_secs(env::var("LOOP_UPDATE_SLEEP").unwrap_or_else(|_| "10".to_string()).parse::<u64>().unwrap())).await;
                }
            });


            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}


pub async fn bot_start() {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => { token }
        Err(_) => { panic!("ENV: DISCORD_TOKEN not set!") }
    };

    let http = Http::new(&token);

    // We will fetch your bots owners and id
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
        StandardFramework::new().configure(|c| c.owners(owners).prefix(env::var("BOT_PREFIX").unwrap_or_else(|_| "~".to_string()))).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}