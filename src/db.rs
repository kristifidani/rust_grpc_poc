use crate::grpc::movie::MovieItem;
use tokio_postgres::{Client, NoTls};
use tonic::Status;

#[derive(Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Status> {
        let db_url = dotenv::var("DB_URL").map_err(|dotenv_error| {
            Status::internal(format!("failed to get url: {:?}", dotenv_error))
        })?;
        let (client, connection) = tokio_postgres::connect(db_url.as_str(), NoTls)
            .await
            .map_err(|db_connection_error| {
                Status::internal(format!(
                    "failed to initialize db: {:?}",
                    db_connection_error
                ))
            })?;
        tokio::spawn(connection);

        Self::create_database(&client).await?;
        Ok(Self { client })
    }

    async fn create_database(client: &Client) -> Result<(), Status> {
        let db_name = dotenv::var("DB_NAME").map_err(|dotenv_error| {
            Status::internal(format!("failed to get db name: {:?}", dotenv_error))
        })?;

        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
              id SERIAL PRIMARY KEY,
              title TEXT NOT NULL,
              year INT NOT NULL,
              genre TEXT NOT NULL
          )",
            db_name
        );

        client
            .batch_execute(&create_table_sql)
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to create table in database {}, {:?}",
                    db_name, db_error
                ))
            })
    }

    pub async fn fetch_movies(&self) -> Result<Vec<MovieItem>, Status> {
        let query = "SELECT * FROM cinema";
        let rows = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| Status::internal(format!("Failed to execute SQL query: {:?}", e)))?;

        let mut movies = Vec::new();

        for row in &rows {
            let movie = MovieItem {
                id: row.get("id"),
                title: row.get("title"),
                year: row.get("year"),
                genre: row.get("genre"),
            };
            movies.push(movie);
        }

        Ok(movies)
    }

    pub async fn create_movie(&self, movie: &MovieItem) -> Result<MovieItem, Status> {
        let db_name = dotenv::var("DB_NAME").map_err(|dotenv_error| {
            Status::internal(format!("failed to get db name: {:?}", dotenv_error))
        })?;

        let statement = self.client
          .prepare(&format!(
              "INSERT INTO {} (title, year, genre) VALUES ($1, $2, $3) RETURNING id, title, year, genre",
              db_name
          ))
          .await
          .map_err(|db_error| {
              Status::internal(format!(
                  "failed to prepare INSERT statement: {:?}",
                  db_error
              ))
          })?;

        let inserted_row = self
            .client
            .query_one(&statement, &[&movie.title, &movie.year, &movie.genre])
            .await
            .map_err(|db_error| {
                Status::internal(format!("failed to insert movie: {:?}", db_error))
            })?;

        Ok(MovieItem {
            id: inserted_row.get("id"),
            title: inserted_row.get("title"),
            year: inserted_row.get("year"),
            genre: inserted_row.get("genre"),
        })
    }
}
