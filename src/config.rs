use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: u32,
  pub service_account: String,
}

#[derive(Debug, Deserialize)]
pub struct Env {
  pub host: String,
  pub port: u32,
}

impl Env {
  pub fn to_address(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}
