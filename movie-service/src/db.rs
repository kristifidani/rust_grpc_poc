use crate::config::{Config, load_configuration};
use crate::grpc::movie::MovieItem;
use tokio_postgres::{Client, NoTls};
use tonic::Status;

fn get_configuration() -> Result<Config, Status> {
    match load_configuration() {
        Ok(env_vars) => Ok(env_vars),
        Err(e) => {
            let custom_error = format!(
                "This program requires environment variables to be defined. \n
        Required variables: \n
        POSTGRES URL -> url for connecting to the database \n
        POSTGRES DATABASE NAME -> database/table name for storing data \n
        The following error occured while loading environment variables: {:?} \n",
                e
            );
            Err(Status::internal(custom_error))
        }
    }
}

#[derive(Debug)]
pub struct DB {
    config: Config,
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Status> {
        let config = get_configuration()?;
        let (client, connection) = tokio_postgres::connect(config.postgres_url.as_str(), NoTls)
            .await
            .map_err(|db_connection_error| {
                Status::internal(format!(
                    "failed to initialize db: {:?}",
                    db_connection_error
                ))
            })?;
        tokio::spawn(connection);

        Self::create_database(&client, &config).await?;
        Ok(Self { config, client })
    }

    async fn create_database(client: &Client, config: &Config) -> Result<(), Status> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
              id SERIAL PRIMARY KEY,
              title TEXT NOT NULL,
              year INT NOT NULL,
              genre TEXT NOT NULL
          )",
            config.postgres_db_name
        );

        client
            .batch_execute(&create_table_sql)
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to create table in database {}, {:?}",
                    config.postgres_db_name, db_error
                ))
            })
    }

    pub async fn fetch_movies(&self) -> Result<Vec<MovieItem>, Status> {
        let query = format!("SELECT * FROM {}", self.config.postgres_db_name);
        let rows = self
            .client
            .query(query.as_str(), &[])
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
        let statement = self.client
          .prepare(&format!(
              "INSERT INTO {} (title, year, genre) VALUES ($1, $2, $3) RETURNING id, title, year, genre",
              self.config.postgres_db_name
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

    pub async fn update_movie(&self, movie: &MovieItem) -> Result<MovieItem, Status> {
        let statement = self.client
          .prepare(&format!(
              "UPDATE {} SET title = $1, year = $2, genre = $3 WHERE id = $4 RETURNING id, title, year, genre",
              self.config.postgres_db_name
          ))
          .await
          .map_err(|db_error| {
              Status::internal(format!(
                  "failed to prepare UPDATE statement: {:?}",
                  db_error
              ))
          })?;

        let updated_row = self
            .client
            .query_one(
                &statement,
                &[&movie.title, &movie.year, &movie.genre, &movie.id],
            )
            .await
            .map_err(|db_error| {
                Status::internal(format!("failed to update movie: {:?}", db_error))
            })?;

        Ok(MovieItem {
            id: updated_row.get("id"),
            title: updated_row.get("title"),
            year: updated_row.get("year"),
            genre: updated_row.get("genre"),
        })
    }

    pub async fn delete_movie(&self, id: i32) -> Result<i32, Status> {
        let statement = self
            .client
            .prepare(&format!(
                "DELETE FROM {} WHERE id = $1 RETURNING id",
                self.config.postgres_db_name
            ))
            .await
            .map_err(|db_error| {
                Status::internal(format!(
                    "failed to prepare DELETE statement: {:?}",
                    db_error
                ))
            })?;

        let deleted_row = self
            .client
            .query_one(&statement, &[&id])
            .await
            .map_err(|db_error| {
                Status::internal(format!("failed to delete movie: {:?}", db_error))
            })?;

        Ok(deleted_row.get("id"))
    }
}
