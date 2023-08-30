use std::sync::Arc;

use dotenv::dotenv;
use futures::StreamExt;
use mongodb::bson::{self, Document};
use mongodb::bson::{doc, oid::ObjectId, Bson};
use mongodb::error::Result as MongoResult;
use mongodb::{options::ClientOptions, Client, Collection, Database};

use crate::grpc::movie::MovieItem;

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let client = DB::create_mongodb_client().await?;
        Ok(Self { client })
    }

    async fn create_mongodb_client() -> mongodb::error::Result<Client> {
        dotenv().ok();
        let mongodb_url = dotenv::var("DB_URL").expect("mongodb url not found");
        let mut client_options = ClientOptions::parse(mongodb_url).await?;
        client_options.app_name = Some("cinema".to_string());
        Client::with_options(client_options)
    }

    fn get_collection(&self) -> Collection<Document> {
        let mongodb_name = dotenv::var("DB_NAME").expect("mongodb name not found");
        let mongodb_collection = dotenv::var("COLLECTION").expect("mongodb collection not found");
        self.client
            .database(mongodb_name.as_str())
            .collection(mongodb_collection.as_str())
    }

    fn convert_db_document_to_movie_item(&self, doc: &Document) -> MongoResult<MovieItem> {
        let id = doc
            .get_object_id("_id")
            .or_else(|_| {
                Err(mongodb::error::ErrorKind::Custom(Arc::new(
                    "Missing '_id' field in document",
                )))
            })?
            .to_hex()
            .parse::<i32>()
            .map_err(|_| {
                mongodb::error::ErrorKind::Custom(Arc::new("Failed to parse 'id' field"))
            })?;

        let title = doc
            .get_str("TITLE")
            .or_else(|_| {
                Err(mongodb::error::ErrorKind::Custom(Arc::new(
                    "Missing 'TITLE' field in document",
                )))
            })?
            .to_owned();

        let year = doc
            .get_i32("YEAR")
            .or_else(|_| {
                Err(mongodb::error::ErrorKind::Custom(Arc::new(
                    "Missing 'YEAR' field in document",
                )))
            })?
            .to_owned();

        let genre = doc
            .get_str("GENRE")
            .or_else(|_| {
                Err(mongodb::error::ErrorKind::Custom(Arc::new(
                    "Missing 'GENRE' field in document",
                )))
            })?
            .to_string();

        Ok(MovieItem {
            title,
            year,
            genre,
        })
    }

    pub async fn get_movies(&self) -> MongoResult<Vec<MovieItem>> {
        let mut cursor = self.get_collection().find(None, None).await?;

        let mut result: Vec<MovieItem> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.convert_db_document_to_movie_item(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_movie(&self, movie: &MovieItem) -> Result<MovieItem, tonic::Status> {
        let record = doc! {
            "title": movie.title.clone(),
            "year": movie.year,
            "genre": movie.genre.clone(),
        };
        match self.get_collection().insert_one(record, None).await {
            Ok(inserted_record) => {
                if let Some(inserted_id) = inserted_record.inserted_id.as_object_id() {
                    println!("{:?}", inserted_id);
                    Ok(MovieItem {
                        title: movie.title.clone(),
                        year: movie.year.clone(),
                        genre: movie.genre.clone(),
                    })
                } else {
                    Err(tonic::Status::internal("Failed to get movie ID"))
                }
            }
            Err(e) => Err(tonic::Status::internal(format!("Failed to create new movie: {:?}", e))),
        }
    }
}
