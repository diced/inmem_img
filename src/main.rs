use dashmap::DashMap;
use std::{env::var, sync::Arc};

use actix_web::{App, HttpServer, web::{self, Data}};
use log::{LevelFilter, info};
use inmem_img::{models::{ImageConfig, State}, routes::{api_get_image, api_upload_image, get_image, not_found}};
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  SimpleLogger::new()
    .with_level(LevelFilter::Debug)
    .with_module_level("rustls", LevelFilter::Off)
    .with_module_level("mio", LevelFilter::Off)
    .with_module_level("actix_server", LevelFilter::Info)
    .init()
    .unwrap();
  let config = Arc::new(check_env());

  info!("config:");
  info!("\tPORT: {}", config.port);
  info!("\tAUTHORIZATION: {}", config.authorization);
  info!("\tRAND_LENGTH: {}", config.rand_length);

  let state: Arc<State> = Arc::new(State {
    stored_images: DashMap::new(),
    config: config.clone()
  });

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(state.clone()))
      .service(api_upload_image)
      .service(api_get_image)
      .service(get_image)
      .default_service(web::get().to(not_found))
  })
  .bind(format!("0.0.0.0:{}", config.port))?
  .run()
  .await
}

fn check_env() -> ImageConfig {
  info!("reading environment...");
  // VARS:
  //  "PORT" will default to 3000
  //  "RAND_LENGTH" will default to 7
  //  "AUTHORIZATION" will be needed

  ImageConfig {
    port: var("PORT").unwrap_or("3000".to_string()).parse().unwrap(),
    authorization: var("AUTHORIZATION").expect("no AUTHORIZATION env var"),
    rand_length: var("RAND_LENGTH").unwrap_or("7".to_string()).parse().unwrap()
  }
}