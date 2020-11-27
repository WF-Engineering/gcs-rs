#[macro_use]
extern crate log;

mod api;
mod api_error;
mod config;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io;

pub use config::{Config, Env};

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let config = envy::from_env::<Config>()
    .map_err(|err| error!("Deserilize config err: {:?}", err))
    .unwrap();

  let env = envy::from_env::<Env>()
    .map_err(|err| error!("Deserilize env err: {:?}", err))
    .unwrap();

  debug!("Running at {}", &env.to_address());

  HttpServer::new(move || {
    App::new()
      .data(config.clone())
      .wrap(middleware::Logger::default())
      .service(web::resource("/upload_object").route(web::post().to(api::upload_object)))
  })
  .bind(&env.to_address())?
  .run()
  .await
}
