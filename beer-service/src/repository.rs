use std::env;

use dotenv::dotenv;
use futures::TryStreamExt;
use log::info;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use mongodb::{Client, Collection};

use crate::model::Beer;

pub struct BeerRepository {
  col: Collection<Beer>,
}

impl BeerRepository {
  pub(crate) async fn init() -> Self {
    dotenv().ok();
    let uri =
      env::var("MONGODB_URI").expect("you need to set a proper MONGODB_URI in your environment");
    let client = Client::with_uri_str(uri)
      .await
      .expect("could not instantiate MongoDB client");
    let dbs = client
      .list_database_names(None, None)
      .await
      .expect("could not obtain any databases")
      .join(",");
    info!("available databases {}", dbs);
    let db = client.database("beersdb");
    let col = db.collection("Beer");
    BeerRepository { col }
  }

  pub(crate) async fn find_all_beers(&self) -> Result<Vec<Beer>> {
    let col = self.col.clone();
    let cursor = col.find(None, None).await?;
    let beers: Vec<Beer> = cursor.try_collect().await?;
    Ok(beers)
  }

  pub(crate) async fn find_beer(&self, id: ObjectId) -> Result<Option<Beer>> {
    let col = self.col.clone();
    let filter = doc! {"_id": id};
    let maybe_beer = col.find_one(filter, None).await?;
    Ok(maybe_beer)
  }

  pub(crate) async fn create_beer(&self, beer: Beer) -> Result<Beer> {
    let col = self.col.clone();
    let created = col.insert_one(beer.clone(), None).await?;
    let new_beer = Beer {
      id: created.inserted_id.as_object_id(),
      ..beer.clone()
    };
    Ok(new_beer)
  }

  pub(crate) async fn delete_beer(&self, id: ObjectId) -> Result<()> {
    let col = self.col.clone();
    let filter = doc! {"_id": id};
    col.delete_one(filter, None).await?;
    Ok(())
  }
}
