use crypto_hash::{hex_digest, Algorithm};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct ErrorData {
  pub error: ErrorInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorInfo {
  pub status: usize,
  pub code: usize,
  pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessData<T> {
  data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
  pub build: String,
  pub developers: String,
  pub timestamp: String,
}

const BASE_URL: &str = "https://api.mixin.one";
pub async fn get<T>(token: String, url: &str) -> Result<T, Box<dyn Error>>
where
  T: serde::de::DeserializeOwned,
{
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let res = client
    .request(
      Request::builder()
        .method(Method::GET)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .uri(format!("{}{}", BASE_URL, url))
        .body(Body::empty())
        .unwrap(),
    )
    .await?;
  let body = res.into_body();
  let body = hyper::body::to_bytes(body).await?;
  let body: SuccessData<T> = serde_json::from_slice(&body.slice(..)).unwrap();
  Ok(body.data)
}

pub async fn post<T>(token: String, url: &str, body: Body) -> Result<T, Box<dyn Error>>
where
  T: serde::de::DeserializeOwned,
{
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let res = client
    .request(
      Request::builder()
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .uri(format!("{}{}", BASE_URL, url))
        .body(body)
        .unwrap(),
    )
    .await?;
  let body = res.into_body();
  let body = hyper::body::to_bytes(body).await?;
  let body: SuccessData<T> = serde_json::from_slice(&body.slice(..)).unwrap();
  Ok(body.data)
}

pub fn sign_request(method: &str, url: &str, body: &str) -> String {
  let payload = format!("{}{}{}", method.to_uppercase(), url, body);
  hex_digest(Algorithm::SHA256, payload.as_bytes())
}
