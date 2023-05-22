use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{error, get, HttpResponse, Responder, Result};
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model::{Beer, PartialBeer};
use crate::repository::BeerRepository;

#[derive(OpenApi)]
#[openapi(
  info(description = "Beer API", license(name = "MIT")),
  paths(),
  components(schemas(Beer, PartialBeer)),
  tags((name = "crate", description = "Beer API endpoints"))
)]
struct ApiDoc;

pub fn configure(db: Data<BeerRepository>) -> impl FnOnce(&mut ServiceConfig) {
  |config| {
    config.app_data(db).service(hello).service(
      SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
  }
}

#[get("/hello/{name}")]
pub async fn hello(path: Path<String>) -> Result<impl Responder> {
  let name = path.into_inner();
  let greetings = format!("Hello, {name}!");
  Ok(HttpResponse::Ok().body(greetings))
}

trait ExtractFromPath {
  fn object_id(self) -> Result<ObjectId>;
}

impl ExtractFromPath for Path<String> {
  fn object_id(self) -> Result<ObjectId> {
    let id_string = self.into_inner();
    let id = ObjectId::parse_str(id_string).map_err(|_| error::ErrorBadRequest("id"))?;
    Ok(id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{test, web, App};

  #[actix_web::test]
  async fn hello_works() {
    let request = test::TestRequest::get().uri("/hello/Chris").to_request();
    let app = test::init_service(App::new().service(hello)).await;

    let response = test::call_and_read_body(&app, request).await;

    assert_eq!(response, web::Bytes::from_static(b"Hello, Chris!"));
  }
}
