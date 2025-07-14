use crate::db::MovieRepoImpl;
use crate::types::grpc::movie::AddMovieRequest;
use crate::types::grpc::movie::{
    DeleteMovieRequest, GetMoviesResponse, MovieItem, movie_server::Movie,
};
use tonic::{Request, Response, Status};

pub struct MovieService<M> {
    pub db: M,
}

#[tonic::async_trait]
impl<M: MovieRepoImpl> Movie for MovieService<M> {
    async fn get_movies(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetMoviesResponse>, Status> {
        let movies = self
            .db
            .fetch_movies()
            .await?
            .into_iter()
            .map(MovieItem::from)
            .collect();

        let reply = GetMoviesResponse { movies };
        Ok(Response::new(reply))
    }

    async fn add_movie(&self, request: Request<AddMovieRequest>) -> Result<Response<()>, Status> {
        let inner_request = request.into_inner();

        let index = uuid::Uuid::new_v4().to_string();
        let new_movie = crate::types::dtos::MovieEntity {
            index,
            title: inner_request.title,
            year: inner_request.year,
            genre: inner_request.genre,
        };
        self.db.create_movie(&new_movie).await?;

        Ok(Response::new(()))
    }

    async fn edit_movie(&self, request: Request<MovieItem>) -> Result<Response<()>, Status> {
        let inner_request = request.into_inner();
        self.db.update_movie(&inner_request.into()).await?;

        Ok(Response::new(()))
    }

    async fn delete_movie(
        &self,
        request: Request<DeleteMovieRequest>,
    ) -> Result<Response<()>, Status> {
        self.db.delete_movie(request.into_inner().index).await?;

        Ok(Response::new(()))
    }
}
