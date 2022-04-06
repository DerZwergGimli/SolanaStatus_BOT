use log::{error, info};
use reqwest::{Error, Response, StatusCode};
use crate::solana::solana_types::Transaction;

static BASE_URL:&str = "https://public-api.solscan.io/";

pub struct SolscanAPI{

}



impl  SolscanAPI {

    pub async fn get_transaction(signature : &str) -> Option<Transaction> {
        let data = transaction_solscan("").await.unwrap();
        info!("{:?}", data);
        Some(data)
    }
}

async fn fetch_solscan(endpoint: String) -> Result<Response, Error> {
    let url = BASE_URL.to_owned() + endpoint.as_str();
    let client = reqwest::Client::new();
    client.get(url.clone())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
}

async fn transaction_solscan(signature: &str) -> Option<Transaction> {
    match fetch_solscan("transaction/".to_owned() + signature).await {
        Ok(resp) => {

            Some(Transaction {
                signature: "".to_string(),
                timestamp: 0,
            })
        },
        _ => None
    }
}