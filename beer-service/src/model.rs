use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Beer {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  #[schema(value_type = Option<String>)]
  pub id: Option<ObjectId>,
  #[schema(example = "Schanzenbräu")]
  pub brand: String,
  #[schema(example = "Schanze Rot")]
  pub name: String,
  #[schema(example = 5.0)]
  pub strength: f64,
}

impl Beer {
  pub(crate) fn from_partial(insert: PartialBeer) -> Self {
    Beer {
      id: None,
      brand: insert.brand,
      name: insert.name,
      strength: insert.strength,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PartialBeer {
  #[schema(example = "Schanzenbräu")]
  pub brand: String,
  #[schema(example = "Schanze Rot")]
  pub name: String,
  #[schema(example = 5.0)]
  pub strength: f64,
}
