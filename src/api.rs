use actix_web::{web, HttpRequest, HttpResponse};
use google_storage1::{Object, Storage};
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use std::{fs, io::prelude::*};
use yup_oauth2 as oauth2;

use super::{api_error::ApiError, config::Config, resp::ImageResponse};

pub async fn upload_object(
  request: HttpRequest,
  payload: web::Bytes,
  config: web::Data<Config>,
) -> Result<HttpResponse, ApiError> {
  let name = request
    .headers()
    .get("Name")
    .and_then(|h| h.to_str().ok())
    .ok_or(ApiError::MissingHeader("Name"))?;

  let bucket = request
    .headers()
    .get("Bucket")
    .and_then(|h| h.to_str().ok())
    .ok_or(ApiError::MissingHeader("Bucket"))?;

  let mime_type = request
    .headers()
    .get("Mime-Type")
    .and_then(|h| h.to_str().ok())
    .ok_or(ApiError::MissingHeader("Mime-Type"))?;

  let saved_path = name;

  let mut file = fs::File::create(saved_path)?;
  file.write_all(&payload)?;

  let client_secret = oauth2::service_account_key_from_file(&config.service_account)
    .map_err(ApiError::ServiceAccountNotFound)?;
  let client = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));

  let authenticator = oauth2::ServiceAccountAccess::new(client_secret, client);
  let client = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));

  let hub = Storage::new(client, authenticator);

  let object = Object::default();
  let (response, object) = hub.objects().insert(object, bucket).name(name).upload(
    fs::File::open(saved_path)?,
    mime_type
      .parse()
      .map_err(|_| ApiError::MimeTypeParsingError)?,
  )?;

  fs::remove_file(saved_path)?;

  if response.status.is_success() {
    let bytes = serde_json::to_vec(&object)?;
    let resp = serde_json::from_slice::<ImageResponse>(&bytes)?;

    Ok(HttpResponse::Ok().json(resp))
  } else {
    Err(ApiError::NotSuccessResponse(response))
  }
}
