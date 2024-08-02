use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512, Digest};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use std::{env};

use cucumber::{given, then, when, World as _};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(cucumber::World, Debug, Default)]
struct World {
    pub url: String,
    pub api_key: String,
    pub private_key: String,

    pub status: Option<u16>,
    pub body: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
struct OrderResponse {
    error: Vec<String>
}

#[given(expr = "Environment variables {string}, {string}, {string} are set")]
async fn check_env_vars(w: &mut World, url_env: String, api_key_env: String, private_key_env: String) {
    let url = env::var(url_env);
    let api_key = env::var(api_key_env);
    let private_key = env::var(private_key_env);

    assert!(url.is_ok(), "Failed to read URL env var: {:?}", url.err());
    assert!(api_key.is_ok(), "Failed to read API_KEY env var: {:?}", api_key.err());
    assert!(private_key.is_ok(), "Failed to read PRIVATE_KEY env var: {:?}", private_key.err());

    w.url = url.unwrap();
    w.api_key = api_key.unwrap();
    w.private_key = private_key.unwrap();
}

#[when(expr = "I send a POST request to {string}")]
async fn send_post_request(w: &mut World, path: String) {
    let url = format!("{}{}", w.url, path);

    let now = Utc::now();

    // Convert the current time to milliseconds since the Unix epoch
    let nonce = now.timestamp_millis();

    let mut payload = HashMap::new();
    payload.insert("nonce".to_string(), nonce.to_string());

    let private_key = w.private_key.clone();
    let b64_decoded_secret = general_purpose::STANDARD.decode(private_key).unwrap();

    let signature = get_signature(&path, &payload, &b64_decoded_secret);

    let client = reqwest::Client::new();

    // Define the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded; charset=utf-8"));
    headers.insert("API-Key", HeaderValue::from_str(&w.api_key).unwrap());
    headers.insert("API-Sign", HeaderValue::from_str(&signature).unwrap());

    // Send the POST request
    let response = client
        .post(url)
        .headers(headers)
        .form(&payload)
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    w.status = Some(status);
    w.body = Some(body);
}

#[then(expr = "the response status should be {int}")]
async fn check_response_status(w: &mut World, expected_status: u16) {
    assert_eq!(w.status.unwrap(), expected_status);
}

#[then(expr = "the response body should contain {int} errors")]
async fn check_response_body(w: &mut World, expected_errors: u16) {
    let body =  <Option<String> as Clone>::clone(&w.body).unwrap();
    let resp: OrderResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(resp.error.len() as u16, expected_errors)
}

fn get_signature(url_path: &str, values: &HashMap<String, String>, secret: &[u8]) -> String {
    // Step 1: SHA256 hash
    let mut sha256 = Sha256::new();
    let nonce = values.get("nonce").expect("nonce missing");
    let encoded_values = serde_urlencoded::to_string(values).unwrap();
    sha256.update(format!("{}{}", nonce, encoded_values).as_bytes());
    let shasum = sha256.finalize();

    // Step 2: HMAC-SHA512 hash
    let mut mac = Hmac::<Sha512>::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(url_path.as_bytes());
    mac.update(&shasum);
    let macsum = mac.finalize().into_bytes();

    // Step 3: Base64 encode
    general_purpose::STANDARD.encode(&macsum)
}


#[tokio::main]
async fn main() {
    World::run("tests/features/private_api.feature").await;
}