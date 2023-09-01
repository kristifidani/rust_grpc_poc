use crate::grpc::movie::MovieItem;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection};
use tonic::Status;

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Status> {
        let client = DB::create_mongodb_client().await?;
        Ok(Self { client })
    }

    async fn create_mongodb_client() -> Result<Client, Status> {
        dotenv().ok();
        let mongodb_url = dotenv::var("DB_URL").map_err(|dotenv_error| {
            Status::internal(format!("failed to get mongo DB URL: {:?}", dotenv_error))
        })?;
        let mongodb_name = dotenv::var("DB_NAME").map_err(|dotenv_error| {
            Status::internal(format!("failed to get mongo DB name: {:?}", dotenv_error))
        })?;
        let mut client_options =
            ClientOptions::parse(mongodb_url)
                .await
                .map_err(|url_parse_error| {
                    Status::internal(format!(
                        "Failed to parse MongoDB URL: {:?}",
                        url_parse_error
                    ))
                })?;
        client_options.app_name = Some(mongodb_name);
        Ok(
            Client::with_options(client_options).map_err(|client_error| {
                Status::internal(format!("Failed to create Mongo client: {:?}", client_error))
            })?,
        )
    }

    fn get_collection(&self) -> Result<Collection<Document>, Status> {
        let mongodb_name = dotenv::var("DB_NAME").map_err(|dotenv_error| {
            Status::internal(format!("failed to get mongo DB name: {:?}", dotenv_error))
        })?;
        let mongodb_collection = dotenv::var("COLLECTION").map_err(|dotenv_error| {
            Status::internal(format!(
                "failed to get mongo collection: {:?}",
                dotenv_error
            ))
        })?;
        Ok(self
            .client
            .database(mongodb_name.as_str())
            .collection(mongodb_collection.as_str()))
    }

    fn convert_db_document_to_movie_item(&self, doc: &Document) -> Result<MovieItem, Status> {
        let id = doc
            .get_object_id("_id")
            .map_err(|_| Status::internal("Missing _id field in document"))?
            .to_hex();

        let title = doc
            .get_str("title")
            .map_err(|e| Status::internal(format!("Missing TITLE field in document: {:?}", e)))?
            .to_owned();

        let year = doc
            .get_i32("year")
            .map_err(|_| Status::internal("Missing YEAR field in document"))?
            .to_owned();

        let genre = doc
            .get_str("genre")
            .map_err(|_| Status::internal("Missing GENRE field in document"))?
            .to_string();

        Ok(MovieItem {
            id,
            title,
            year,
            genre,
        })
    }

    pub async fn get_movies(&self) -> Result<Vec<MovieItem>, Status> {
        let mut cursor = self
            .get_collection()?
            .find(None, None)
            .await
            .map_err(|fetch_error| {
                Status::internal(format!("could not find documents: {:?}", fetch_error))
            })?;

        let mut result: Vec<MovieItem> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.convert_db_document_to_movie_item(&doc.map_err(
                |cursor_error| {
                    Status::internal(format!("failed to get document: {:?}", cursor_error))
                },
            )?)?);
        }
        Ok(result)
    }

    pub async fn create_movie(&self, movie: &MovieItem) -> Result<MovieItem, Status> {
        let record = doc! {
            "title": movie.title.clone(),
            "year": movie.year,
            "genre": movie.genre.clone(),
        };
        match self.get_collection()?.insert_one(record, None).await {
            Ok(inserted_record) => {
                if let Some(inserted_id) = inserted_record.inserted_id.as_object_id() {
                    Ok(MovieItem {
                        id: inserted_id.to_string(),
                        title: movie.title.clone(),
                        year: movie.year.clone(),
                        genre: movie.genre.clone(),
                    })
                } else {
                    Err(Status::internal("Failed to get movie ID"))
                }
            }
            Err(insertion_error) => Err(Status::internal(format!(
                "Failed to create new movie: {:?}",
                insertion_error
            ))),
        }
    }
}
