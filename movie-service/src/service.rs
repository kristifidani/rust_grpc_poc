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
        let movies = self.db.fetch_movies().await?;

        let reply = GetMoviesResponse { movies };
        Ok(Response::new(reply))
    }

    async fn add_movie(&self, request: Request<AddMovieRequest>) -> Result<Response<()>, Status> {
        let inner_request = request.into_inner();

        let index = uuid::Uuid::new_v4().to_string();
        let new_movie = MovieItem {
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
        self.db.update_movie(&inner_request).await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::MockMovieRepoImpl, types::grpc::movie::AddMovieRequest};

    const INDEX: &str = "bd24fa48-690d-476e-b9c9-e6a746f02941";
    const TITLE: &str = "Unit Test Movie Title";

    #[tokio::test]
    async fn fetch_movies_utest() {
        let mut mock_repo = MockMovieRepoImpl::new();
        mock_repo.expect_fetch_movies().returning(|| {
            Ok(vec![MovieItem {
                index: INDEX.to_string(),
                title: TITLE.to_string(),
                year: 2025,
                genre: "Action".to_string(),
            }])
        });

        let service = MovieService { db: mock_repo };
        let request = Request::new(());

        let response = service.get_movies(request).await;
        let movies = response.unwrap().into_inner().movies;
        assert_eq!(movies.len(), 1);
        assert_eq!(movies[0].title, TITLE);
    }

    #[tokio::test]
    async fn add_movie_utest() {
        let mut mock_repo = MockMovieRepoImpl::new();
        mock_repo.expect_create_movie().returning(|_m| Ok(()));

        let service = MovieService { db: mock_repo };
        let request = Request::new(AddMovieRequest {
            title: TITLE.to_string(),
            year: 2025,
            genre: "Action".to_string(),
        });

        let response = service.add_movie(request).await;
        response.unwrap();
    }

    #[tokio::test]
    async fn edit_movie_utest() {
        let mut mock_repo = MockMovieRepoImpl::new();
        mock_repo.expect_update_movie().returning(|_m| Ok(()));

        let service = MovieService { db: mock_repo };
        let request = Request::new(MovieItem {
            index: INDEX.to_string(),
            title: TITLE.to_string(),
            year: 2025,
            genre: "Action".to_string(),
        });

        let response = service.edit_movie(request).await;
        response.unwrap();
    }

    #[tokio::test]
    async fn delete_movie_utest() {
        let mut mock_repo = MockMovieRepoImpl::new();
        mock_repo.expect_delete_movie().returning(|_index| Ok(()));

        let service = MovieService { db: mock_repo };
        let request = Request::new(DeleteMovieRequest {
            index: INDEX.to_string(),
        });

        let response = service.delete_movie(request).await;
        response.unwrap();
    }
}
