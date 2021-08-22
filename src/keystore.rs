pub struct Keystore {
  pub client_id: String,
  pub client_secret: String,
  pub session_id: String,
  pub private_key: String,
  pub pin_token: String,
  pub scope: String,
  pub pin: String,
}

use serde_json::json;
extern crate frank_jwt;
use frank_jwt::{encode, Algorithm};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
impl Keystore {
  pub fn new(key: Keystore) -> Keystore {
    key
  }
  pub fn get_sign(&self, signatrue: String) -> Result<String, Box<dyn Error>> {
    let iat = get_now();
    let exp = iat + 3600;
    let jit = Uuid::new_v4().to_string();
    let payload = json!({
      "uid": self.client_id,
      "sid": self.session_id,
      "iat": iat,
      "exp": exp,
      "jit": jit,
      "sig": signatrue,
      "scp": self.scope,
    });
    let header = json!({
      "typ": "JWT",
      "alg": "RS512"
    });
    let jwt = encode(header, &self.private_key, &payload, Algorithm::RS512)?;
    Ok(jwt)
  }
}

fn get_now() -> usize {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  let ms = since_the_epoch.as_secs() as usize
    + (since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0) as usize;
  ms
}
