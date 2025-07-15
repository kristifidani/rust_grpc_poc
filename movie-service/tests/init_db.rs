use movie_grpc_service::db::DB_NAME;
use std::env;
use tokio_postgres::NoTls;

#[tokio::test]
async fn test_initialize_and_populate_db() {
    dotenvy::dotenv().ok();

    // Connect to the database
    let db_connection_string = env::var("DB_URL").expect("DB_URL must be set");

    let (client, connection) = tokio_postgres::connect(&db_connection_string, NoTls)
        .await
        .expect("Failed to connect to database");

    // Spawn the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Database connection error: {}", e);
        }
    });

    // Create table
    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            index TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            year INT NOT NULL,
            genre TEXT NOT NULL
        )",
        DB_NAME
    );

    client
        .execute(create_table_sql.as_str(), &[])
        .await
        .expect("Failed to create table");

    // Clean the table first
    let delete_sql = format!("DELETE FROM {}", DB_NAME);
    client
        .execute(&delete_sql, &[])
        .await
        .expect("Failed to clear table before test");

    // Insert two movies
    let insert_sql = format!(
        "INSERT INTO {} (index, title, year, genre) VALUES ($1, $2, $3, $4)",
        DB_NAME
    );

    let movie1 = (
        "bd24fa48-690d-476e-b9c9-e6a746f02941".to_string(),
        "Interstellar",
        2014,
        "Sci-Fi",
    );
    let movie2 = (
        "6ad0f878-0e95-49a1-8767-ecbcb4d9147c".to_string(),
        "Whiplash",
        2014,
        "Drama",
    );

    client
        .execute(&insert_sql, &[&movie1.0, &movie1.1, &movie1.2, &movie1.3])
        .await
        .expect("Failed to insert movie 1");

    client
        .execute(&insert_sql, &[&movie2.0, &movie2.1, &movie2.2, &movie2.3])
        .await
        .expect("Failed to insert movie 2");
}
