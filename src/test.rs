// // src/tests.rs
// use super::*;
// use hyper::{Client, Body, Request};
// // use mockito::{mock, server_url};
// use serde_json::json;

// #[tokio::test]
// async fn test_fetch_prices() {
//     // Mock the external API request
//     let _mock = mock("GET", "/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(r#"{
//             "bitcoin": { "usd": 20000.0 },
//             "ethereum": { "usd": 1000.0 }
//         }"#)
//         .create();

//     // Update the URI to use the mock server
//     let uri = format!("{}/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd", server_url());
//     let client = Client::new();
    
//     // Create a mock request to test
//     let req = Request::builder()
//         .method("GET")
//         .uri(uri)
//         .body(Body::empty())
//         .expect("Failed to build request");

//     let res = client.request(req).await.unwrap();
//     let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
//     let result: PriceResponse = serde_json::from_slice(&body).unwrap();

//     assert_eq!(result.bitcoin.usd, 20000.0);
//     assert_eq!(result.ethereum.usd, 1000.0);
// }

// #[tokio::test]
// async fn test_handle_advance() {
//     // Mock the external API request
//     let _mock = mock("POST", "/finish")
//         .with_status(202)
//         .create();

//     let request = json!({
//         "data": {
//             "payload": {}
//         }
//     });

//     let client = Client::new();
//     let server_addr = server_url();
//     let result = handle_advance(&client, &server_addr, request).await;

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), "accept");
// }

// #[tokio::test]
// async fn test_handle_inspect() {
//     // Mock the external API request
//     let _mock = mock("POST", "/finish")
//         .with_status(202)
//         .create();

//     let request = json!({
//         "data": {
//             "payload": {}
//         }
//     });

//     let client = Client::new();
//     let server_addr = server_url();
//     let result = handle_inspect(&client, &server_addr, request).await;

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), "accept");
// }
