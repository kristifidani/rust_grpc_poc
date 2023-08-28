use crate::grpc::movie::{movie_server::Movie, MovieItem, MovieRequest, MovieResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MovieService {}

#[tonic::async_trait]
impl Movie for MovieService {
    async fn get_movies(
        &self,
        request: Request<MovieRequest>,
    ) -> Result<Response<MovieResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut movies = Vec::new();
        movies.push(MovieItem {
            id: 1,
            title: "Matrix".to_string(),
            year: 1999,
            genre: "Sci-Fi".to_string(),
        });
        movies.push(MovieItem {
            id: 2,
            title: "Spider-Man: Across the Spider-Verse".to_string(),
            year: 2023,
            genre: "Animation".to_string(),
        });
        movies.push(MovieItem {
            id: 3,
            title: "Her".to_string(),
            year: 2013,
            genre: "Drama".to_string(),
        });

        let reply = MovieResponse { movies: movies };

        Ok(Response::new(reply))
    }
}
