use crate::types::dtos::{DB_NAME, MovieEntity};
use tokio_postgres::{Client, NoTls};
use tonic::Status;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MovieRepoImpl: Send + Sync + 'static {
    async fn fetch_movies(&self) -> Result<Vec<MovieEntity>, Status>;
    async fn create_movie(&self, movie: &MovieEntity) -> Result<(), Status>;
    async fn update_movie(&self, movie: &MovieEntity) -> Result<(), Status>;
    async fn delete_movie(&self, id: String) -> Result<(), Status>;
}

pub struct MovieRepo {
    client: Client,
}

impl MovieRepo {
    pub async fn init(url: &str) -> Result<Self, Status> {
        let (client, connection) =
            tokio_postgres::connect(url, NoTls)
                .await
                .map_err(|db_connection_error| {
                    Status::internal(format!(
                        "failed to initialize db: {:?}",
                        db_connection_error
                    ))
                })?;

        // Spawn the connection to run in the background
        tokio::spawn(connection);

        // Ping the database with a basic query to verify the connection
        client
            .query_one("SELECT 1", &[])
            .await
            .map_err(|err| Status::internal(format!("DB ping failed: {:?}", err)))?;

        println!("✅ Successfully connected to PostgreSQL at {}", url);

        // Create database
        Self::create_database(&client).await?;

        Ok(Self { client })
    }

    async fn create_database(client: &Client) -> Result<(), Status> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            index TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            year INT NOT NULL,
            genre TEXT NOT NULL
        )",
            DB_NAME
        );

        client
            .batch_execute(&create_table_sql)
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to create table in database {}, {:?}",
                    DB_NAME, db_error
                ))
            })
    }
}

#[async_trait::async_trait]
impl MovieRepoImpl for MovieRepo {
    async fn fetch_movies(&self) -> Result<Vec<MovieEntity>, Status> {
        let query = format!("SELECT * FROM {}", DB_NAME);
        let rows = self
            .client
            .query(query.as_str(), &[])
            .await
            .map_err(|e| Status::internal(format!("Failed to execute SQL query: {:?}", e)))?;

        let mut movies = Vec::new();

        for row in &rows {
            let movie = MovieEntity {
                index: row.get("index"),
                title: row.get("title"),
                year: row.get("year"),
                genre: row.get("genre"),
            };
            movies.push(movie);
        }

        Ok(movies)
    }

    async fn create_movie(&self, movie: &MovieEntity) -> Result<(), Status> {
        let statement = self
            .client
            .prepare(&format!(
                "INSERT INTO {} (index, title, year, genre) VALUES ($1, $2, $3, $4)",
                DB_NAME
            ))
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to prepare INSERT statement: {:?}",
                    db_error
                ))
            })?;

        self.client
            .execute(
                &statement,
                &[&movie.index, &movie.title, &movie.year, &movie.genre],
            )
            .await
            .map_err(|db_error| {
                Status::internal(format!("failed to insert movie: {:?}", db_error))
            })?;

        Ok(())
    }

    async fn update_movie(&self, movie: &MovieEntity) -> Result<(), Status> {
        let statement = self
            .client
            .prepare(&format!(
                "UPDATE {} SET title = $1, year = $2, genre = $3 WHERE index = $4",
                DB_NAME
            ))
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to prepare UPDATE statement: {:?}",
                    db_error
                ))
            })?;

        let _updated_row = self
            .client
            .execute(
                &statement,
                &[&movie.title, &movie.year, &movie.genre, &movie.index],
            )
            .await
            .map_err(|db_error| {
                Status::internal(format!("failed to update movie: {:?}", db_error))
            })?;

        Ok(())
    }

    async fn delete_movie(&self, index: String) -> Result<(), Status> {
        let statement = self
            .client
            .prepare(&format!("DELETE FROM {} WHERE index = $1", DB_NAME))
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to prepare DELETE statement: {:?}",
                    db_error
                ))
            })?;

        let _deleted_row =
            self.client
                .execute(&statement, &[&index])
                .await
                .map_err(|db_error| {
                    Status::internal(format!("failed to delete movie: {:?}", db_error))
                })?;

        Ok(())
    }
}
