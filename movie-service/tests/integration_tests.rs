use std::env;

use movie_grpc_service::{
    db::MovieRepo,
    service::MovieService,
    types::grpc::movie::{
        AddMovieRequest, DeleteMovieRequest, MovieItem, movie_client::MovieClient,
        movie_server::MovieServer,
    },
};
use tonic::{
    Request,
    transport::{Channel, Endpoint, Server, Uri},
};
use tower::service_fn;

async fn init_movie_service() -> MovieClient<Channel> {
    dotenvy::dotenv().ok();

    // clients
    let (client, server) = tokio::io::duplex(1024);
    let db_connection_string = env::var("DB_URL").expect("DB_URL must be set");

    // repos
    let movie_repo = MovieRepo::init(&db_connection_string)
        .await
        .expect("Failed to initialize database");

    let movie_service = MovieService { db: movie_repo };

    // server
    let _ = Server::builder()
        .add_service(MovieServer::new(movie_service))
        .serve_with_incoming(tokio_stream::iter(vec![Ok::<_, std::io::Error>(server)]))
        .await;

    // communication channel
    let client = hyper_util::rt::tokio::TokioIo::new(client);
    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client already taken",
                    ))
                }
            }
        }))
        .await
        .unwrap();

    let client: MovieClient<Channel> = MovieClient::new(channel);
    client
}

#[tokio::test]
async fn get_movies() {
    // setup
    let mut service = init_movie_service().await;
    // req
    let request = Request::new(());
    // res
    let result = service.get_movies(request).await;
    // assertions
    let inner_res = result.unwrap().into_inner();
    assert!(!inner_res.movies.is_empty());
}

#[tokio::test]
async fn create_new_movie() {
    // setup
    let mut service = init_movie_service().await;
    // req
    let new_movie = AddMovieRequest {
        title: "New Movie Title".to_string(),
        year: 2025,
        genre: "Action".to_string(),
    };
    let request = Request::new(new_movie.clone());
    // res
    let result = service.add_movie(request).await;
    // assertions
    result.unwrap();
}

#[tokio::test]
async fn edit_movie() {
    // setup
    let mut service = init_movie_service().await;
    // req
    let new_movie = MovieItem {
        index: "bd24fa48-690d-476e-b9c9-e6a746f02941".to_string(),
        title: "Edited Movie Title".to_string(),
        year: 2026,
        genre: "Drama".to_string(),
    };
    let request = Request::new(new_movie.clone());
    // res
    let result = service.edit_movie(request).await;
    // assertions
    result.unwrap();
}

#[tokio::test]
async fn delete_movie() {
    // setup
    let mut service = init_movie_service().await;
    // req
    let delete_movie_id = DeleteMovieRequest {
        index: "6ad0f878-0e95-49a1-8767-ecbcb4d9147c".to_string(),
    };
    let request = Request::new(delete_movie_id);
    //res
    let result = service.delete_movie(request).await;
    // assertions
    result.unwrap();
}
