use crate::db::DB;
use crate::grpc::movie::{
    movie_server::Movie, DeleteMovieRr, EditMovieResponse, GetMovieRequest, GetMovieResponse,
    MovieItem,
};
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
        _request: Request<GetMovieRequest>,
    ) -> Result<Response<GetMovieResponse>, Status> {
        let movies = self.db.fetch_movies().await?;
        let reply = GetMovieResponse { movies };
        Ok(Response::new(reply))
    }

    async fn add_movie(
        &self,
        request: Request<MovieItem>,
    ) -> Result<Response<GetMovieResponse>, Status> {
        let new_movie = request.into_inner();
        let movie = self.db.create_movie(&new_movie).await?;
        let reply = GetMovieResponse {
            movies: vec![movie],
        };
        Ok(Response::new(reply))
    }

    async fn edit_movie(
        &self,
        request: Request<MovieItem>,
    ) -> Result<Response<EditMovieResponse>, Status> {
        let update_movie = request.into_inner();
        let eddited_movie = self.db.update_movie(&update_movie).await?;
        let reply = EditMovieResponse {
            movie: Some(eddited_movie),
        };
        Ok(Response::new(reply))
    }

    async fn delete_movie(
        &self,
        request: Request<DeleteMovieRr>,
    ) -> Result<Response<DeleteMovieRr>, Status> {
        let deleted_movie_id = self.db.delete_movie(request.into_inner().id).await?;
        let reply = DeleteMovieRr {
            id: deleted_movie_id,
        };
        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db,
        grpc::movie::{movie_server::Movie, GetMovieRequest},
    };
    use tonic::Request;

    #[tokio::test]
    async fn get_movies_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb"));
        let request = Request::new(GetMovieRequest {});
        let result = movie_service.get_movies(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn add_movie_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb"));
        let new_movie = MovieItem {
            id: 1,
            title: "New Movie Title".to_string(),
            year: 2023,
            genre: "Action".to_string(),
        };
        let request = Request::new(new_movie.clone());

        let result = movie_service.add_movie(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn edit_movie_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb"));
        let new_movie = MovieItem {
            id: 1,
            title: "Edited Movie Title".to_string(),
            year: 2024,
            genre: "Drama".to_string(),
        };
        let request = Request::new(new_movie.clone());

        let result = movie_service.edit_movie(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_movie_utest() {
        let movie_service =
            MovieService::new(db::DB::init().await.expect("failed to initialize mongodb"));

        let delete_movie_id = DeleteMovieRr { id: 1 };
        let request = Request::new(delete_movie_id);

        let result = movie_service.delete_movie(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
