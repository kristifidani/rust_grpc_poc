use std::sync::Arc;

use crate::db::DB;
use crate::grpc::movie::{movie_server::Movie, MovieItem, MovieRequest, MovieResponse};
use tonic::{Request, Response, Status};

pub struct MovieService {
  db: DB,
}

impl MovieService {
  pub fn new(db: DB) -> Self {
      Self { db }
  }
}

#[tonic::async_trait]
impl Movie for MovieService {
    async fn get_movies(
        &self,
        _request: Request<MovieRequest>,
    ) -> Result<Response<MovieResponse>, tonic::Status> {
        let movies = match self.db.get_movies().await {
            Ok(movies) => movies,
            Err(e) => return Err(tonic::Status::internal(format!("Failed to fetch movies: {:?}", e.kind.to_string()))),
        };
        let reply = MovieResponse { movies };
        Ok(Response::new(reply))
    }

    async fn add_movie(
        &self,
        request: Request<MovieItem>,
    ) -> Result<Response<MovieResponse>, Status> {
        let new_movie = request.into_inner();
        let movie = self.db.create_movie(&new_movie).await?;
        let reply = MovieResponse { movies: vec![movie] };
        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db,
        grpc::movie::{movie_server::Movie, MovieRequest},
    };
    use tonic::Request;

    #[tokio::test]
    async fn get_movies_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb").into());
        let request = Request::new(MovieRequest {});
        let result = movie_service.get_movies(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn add_movie_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb").into());
        let new_movie = MovieItem {
            title: "New Movie Title".to_string(),
            year: 2023,
            genre: "Action".to_string(),
        };
        let request = Request::new(new_movie.clone());

        let result = movie_service
            .add_movie(request)
            .await
            .expect("Failed to add movie");

        println!("result: {:?}", result);
        assert_eq!(result.into_inner().movies, vec![new_movie]);
    }
}
