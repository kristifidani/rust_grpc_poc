use std::env;

use movie_grpc_service::{db::DB, grpc::movie::movie_server::MovieServer, movie::MovieService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::init().await?;

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("127.0.0.1:{}", port).parse()?;

    println!("✅ Server running successfully on http://{}", addr);

    Server::builder()
        .add_service(MovieServer::new(MovieService::new(db)))
        .serve(addr)
        .await?;

    Ok(())
}
