mod test;

use json::{object, JsonValue};
use std::env;
use hyper::{Client, Method, Request, Body};
use hyper::header::CONTENT_TYPE;
use hyper::Uri;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PriceResponse {
    bitcoin: Cryptocurrency,
    ethereum: Cryptocurrency,
    // Add other cryptocurrencies as needed
}

#[derive(Deserialize)]
pub struct Cryptocurrency {
    usd: f64,
    // Add other currencies if needed
}

// Function to fetch cryptocurrency prices from an API
pub async fn fetch_prices(client: &Client<hyper::client::HttpConnector>) -> Result<PriceResponse, Box<dyn std::error::Error>> {
    let uri = Uri::from_static("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd");

    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::empty())?;
    
    let res = client.request(req).await?;
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let prices: PriceResponse = serde_json::from_slice(&body)?;

    Ok(prices)
}

// Handle advance state request
pub async fn handle_advance(
    client: &Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data: {}", &request);

    let prices = fetch_prices(client).await?;
    println!("Fetched prices: BTC = ${}, ETH = ${}", prices.bitcoin.usd, prices.ethereum.usd);

    // Update the on-chain state with the new prices
    // This is a placeholder for actual state update logic
    // TODO: Implement logic to update the contract state

    Ok("accept")
}

// Handle inspect state request
pub async fn handle_inspect(
    _client: &Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data: {}", &request);

    // Implement logic to return the current state
    // For example, you might want to return the latest prices
    // This is a placeholder for actual inspection logic
    // TODO: Implement logic to return current state

    Ok("accept")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = "accept";
    loop {
        println!("Sending finish");
        let response = object! {"status" => status};
        let request = Request::builder()
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(Body::from(response.dump()))?;
        let response = client.request(request).await?;
        println!("Received finish status: {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;
            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}
