mod config;
mod db;
mod grpc;
mod movie;

use crate::grpc::movie::movie_server::MovieServer;
use crate::movie::MovieService;
use db::DB;
use std::env;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::init().await?;
    let (mut health_reporter, health_service) = health_reporter();
    health_reporter
        .set_serving::<MovieServer<MovieService>>()
        .await;

    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(tonic_web::enable(MovieServer::new(MovieService::new(db))))
        .serve(format!("0.0.0.0:{}", env::var("PORT").unwrap_or("8080".to_string())).parse()?)
        .await?;

    Ok(())
}
