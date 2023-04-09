use actix_web::http::header::LOCATION;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{delete, error, get, post, put, HttpResponse, Responder, Result};
use futures::TryFutureExt;
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model::{Beer, PartialBeer};
use crate::repository::BeerRepository;

#[derive(OpenApi)]
#[openapi(
  info(description = "Beer API", license(name = "MIT")),
  paths(all_beers, single_beer, create_beer, update_beer, delete_beer),
  components(schemas(Beer, PartialBeer)),
  tags((name = "crate", description = "Beer API endpoints"))
)]
struct ApiDoc;

pub fn configure(db: Data<BeerRepository>) -> impl FnOnce(&mut ServiceConfig) {
  |config| {
    config
      .app_data(db)
      .service(all_beers)
      .service(single_beer)
      .service(create_beer)
      .service(update_beer)
      .service(delete_beer)
      .service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
      );
  }
}

#[utoipa::path(responses(
  (status = 200, description = "list of beers", body = [Beer]),
))]
#[get("/beers")]
pub async fn all_beers(db: Data<BeerRepository>) -> Result<impl Responder> {
  let beers = db
    .find_all_beers()
    .await
    .map_err(error::ErrorInternalServerError)?;
  Ok(HttpResponse::Ok().json(beers))
}

#[utoipa::path(
  responses(
    (status = 200, description = "found beer", body = Beer),
    (status = 204, description = "beer with the specified id does not exist"),
    (status = 400, description = "id is invalid"),
  ),
  params(
    ("id" = String, Path, description = "id of the beer"),
  )
)]
#[get("/beers/{id}")]
pub async fn single_beer(db: Data<BeerRepository>, path: Path<String>) -> Result<impl Responder> {
  let id = path.object_id()?;
  let beer = db
    .find_beer(id)
    .await
    .map_err(error::ErrorInternalServerError)?;
  Ok(match beer {
    None => HttpResponse::NoContent().finish(),
    Some(value) => HttpResponse::Ok().json(value),
  })
}

#[utoipa::path(
  responses(
    (status = 204, description = "beer was deleted"),
    (status = 400, description = "id is invalid"),
    ),
  params(
    ("id" = String, Path, description = "id of the beer"),
  )
)]
#[delete("/beers/{id}")]
pub async fn delete_beer(db: Data<BeerRepository>, path: Path<String>) -> Result<impl Responder> {
  let id = path.object_id()?;
  db.delete_beer(id)
    .await
    .map_err(error::ErrorInternalServerError)?;
  Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
  responses(
    (status = 201, description = "created beer", body = Beer),
  ),
  request_body(content = PartialBeer, description = "beer to create"),
)]
#[post("/beers")]
pub async fn create_beer(
  db: Data<BeerRepository>,
  body: Json<PartialBeer>,
) -> Result<impl Responder> {
  let beer = Beer::from_partial(body.into_inner());
  let created = db
    .create_beer(beer)
    .await
    .map_err(error::ErrorInternalServerError)?;
  let id = created.id.unwrap().to_string();
  Ok(
    HttpResponse::Created()
      .insert_header((LOCATION, format!("/beers/{id}")))
      .json(created),
  )
}

#[utoipa::path(
responses(
    (status = 200, description = "beer was updated"),
    (status = 204, description = "beer does not exist"),
    (status = 400, description = "id is invalid"),
    ),
  params(
    ("id" = String, Path, description = "id of the beer"),
  ),
  request_body(content = PartialBeer, description = "beer to update"),
)]
#[put("/beers/{id}")]
pub async fn update_beer(
  db: Data<BeerRepository>,
  path: Path<String>,
  body: Json<PartialBeer>,
) -> Result<impl Responder> {
  let id = path.object_id()?;
  let beer = Beer {
    id: Some(id),
    ..Beer::from_partial(body.into_inner())
  };
  let updated = db
    .update_beer(beer)
    .map_err(error::ErrorInternalServerError)
    .await?;
  Ok(match updated {
    None => HttpResponse::NoContent().finish(),
    Some(beer) => HttpResponse::Ok().json(beer),
  })
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
