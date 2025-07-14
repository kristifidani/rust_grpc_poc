use std::env;

use movie_grpc_service::{
    db::MovieRepo, service::MovieService, types::grpc::movie::movie_server::MovieServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();
    let db_connection_string = env::var("DB_URL").expect("DB_URL must be set");
    let db = MovieRepo::init(&db_connection_string).await?;

    let movie_service = MovieService { db };

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("127.0.0.1:{}", port).parse()?;
    println!("✅ Server running successfully on http://{}", addr);

    Server::builder()
        .add_service(MovieServer::new(movie_service))
        .serve(addr)
        .await?;

    Ok(())
}
