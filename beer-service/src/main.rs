use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

use crate::repository::BeerRepository;

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));
  let address = ("localhost", 8080);
  info!("starting server on {address:?}");
  let db = BeerRepository::init().await;
  let db_data = Data::new(db);
  HttpServer::new(move || {
    App::new()
      .wrap(Cors::permissive())
      .wrap(Logger::default())
      .configure(api::configure(db_data.clone()))
  })
  .bind(address)?
  .run()
  .await
}
