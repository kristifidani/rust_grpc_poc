use dotenv::dotenv;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub postgres_url: String,
    pub postgres_db_name: String,
}

pub fn load_configuration() -> Result<Config, Box<dyn Error>> {
    dotenv().ok();

    let postgres_url = dotenv::var("DB_URL");
    let postgres_db_name = dotenv::var("DB_NAME");

    if postgres_url.is_err() && postgres_db_name.is_err() {
        Err("Postgres URL and NAME not found".to_string())?
    } else {
        match postgres_url {
            Ok(url) => match postgres_db_name {
                Ok(name) => {
                    let env_vars = Config {
                        postgres_url: url,
                        postgres_db_name: name,
                    };
                    Ok(env_vars)
                }
                Err(e) => Err(format!("Postgres DB Name error: {}", e))?,
            },
            Err(e) => Err(format!("Postgres URL error: {}", e))?,
        }
    }
}
