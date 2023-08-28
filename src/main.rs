mod grpc;
mod movie;

use crate::grpc::movie::movie_server::MovieServer;
use crate::movie::MovieService;
use std::env;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = health_reporter();
    health_reporter
        .set_serving::<MovieServer<MovieService>>()
        .await;

    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(tonic_web::enable(MovieServer::new(MovieService::default())))
        .serve(format!("0.0.0.0:{}", env::var("PORT").unwrap_or("8080".to_string())).parse()?)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::movie::{movie_server::Movie, MovieRequest};
    use tonic::Request;

    #[tokio::test]
    async fn test_get_movies() {
        let movie_service = MovieService::default();
        let request = Request::new(MovieRequest {});
        let result = movie_service.get_movies(request).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
