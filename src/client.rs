use actix_web::{client::Client as Awc, dev::Body};
use bytes::Bytes;

use super::resp::ImageResponse;

pub struct Client {
  server_url: String,
}

impl Client {
  pub fn new(server_url: String) -> Self {
    Self { server_url }
  }

  pub async fn upload_object(
    &self,
    name: String,
    bucket: String,
    mime_type: String,
    payload: Bytes,
  ) -> Result<ImageResponse, String> {
    let client = Awc::build()
      .header("Name", name)
      .header("Bucket", bucket)
      .header("Mime-Type", mime_type)
      .finish();

    let url = format!("{}/upload_object", self.server_url);

    let body = Body::Bytes(payload);
    let resp = client
      .post(url)
      .send_body(body)
      .await
      .unwrap()
      .json::<ImageResponse>()
      .await
      .unwrap();

    Ok(resp)
  }
}
