pub mod keystore;
pub mod request;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::error::Error;
pub struct Client {
  pub keystore: keystore::Keystore,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub user_id: String,
  pub full_name: String,
  pub identity_number: String,
  pub phone: String,
  pub biography: String,
  pub avatar_url: String,
  pub relationship: String,
  pub mute_until: String,
  pub created_at: String,
  pub is_verified: bool,
  pub session_id: String,
  pub pin_token: String,
  pub pin_token_base64: String,
  pub code_id: String,
  pub code_url: String,
  pub has_pin: bool,
  pub has_emergency_contact: bool,
  pub receive_message_source: String,
  pub accept_conversation_source: String,
  pub accept_search_source: String,
  pub fiat_currency: String,
  pub device_status: String,
}

impl Client {
  pub fn new(ks: keystore::Keystore) -> Client {
    Client { keystore: ks }
  }
  pub async fn user_me(&self) -> Result<User, Box<dyn Error>> {
    let u: User = self.get("/me").await?;
    Ok(u)
  }

  pub async fn get<T>(&self, url: &str) -> Result<T, Box<dyn Error>>
  where
    T: serde::de::DeserializeOwned,
  {
    let sig = request::sign_request("GET", url, "");
    let token = self.keystore.get_sign(sig)?;
    let u: T = request::get(token, url).await?;
    Ok(u)
  }

  pub async fn post<T>(&self, url: &str, data: Body) -> Result<T, Box<dyn Error>>
  where
    T: serde::de::DeserializeOwned,
  {
    let sig = request::sign_request("GET", url, "");
    let token = self.keystore.get_sign(sig)?;
    let u: T = request::post(token, url, data).await?;
    Ok(u)
  }
}
