mod config;
mod db;
mod grpc;
mod movie;

use crate::grpc::movie::movie_server::MovieServer;
use crate::movie::MovieService;
use db::DB;
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::init().await?;

    Server::builder()
        .add_service(MovieServer::new(MovieService::new(db)))
        .serve(format!("0.0.0.0:{}", env::var("PORT").unwrap_or("8080".to_string())).parse()?)
        .await?;

    Ok(())
}
