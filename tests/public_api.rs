use std::env;

use cucumber::{given, then, when, World as _};
use serde::{Deserialize, Serialize};

#[derive(cucumber::World, Debug, Default)]
struct World {
    pub url: String,

    pub status: Option<u16>,
    pub body: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    error: Vec<String>
}


#[given(expr = "Environment variable {string} is set")]
async fn check_env_vars(w: &mut World, url_env: String) {
    let url = env::var(url_env);

    assert!(url.is_ok(), "Failed to read URL env var: {:?}", url.err());

    w.url = url.unwrap();
}

#[when(expr = "I send a GET request to {string}")]
async fn request_time(w: &mut World, path: String) {
    let url = format!("{}{}", w.url, path);

    let response = reqwest::get(&url).await.unwrap();
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
    let resp: Response = serde_json::from_str(&body).unwrap();

    assert_eq!(resp.error.len() as u16, expected_errors)
}


#[tokio::main]
async fn main() {
    World::run("tests/features/public_api.feature").await;
}