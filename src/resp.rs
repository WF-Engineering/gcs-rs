use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageResponse {
  pub id: String,
  pub bucket: String,
  pub size: String,
  pub time_created: DateTime<Utc>,
  pub md5_hash: String,
  pub content_type: String,
  pub name: String,
  pub self_link: String,
}
