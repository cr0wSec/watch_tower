use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use futures::lock::Mutex;
use once_cell::sync::Lazy;

pub struct ServerConfig {
    pub crypto_pepper: Vec<u8>,
    pub secret_key: Vec<u8>,
    pub connection: Mutex<PgConnection>,
}

impl ServerConfig {
    fn new() -> Self {
        // load env variables
        dotenv().ok();

        // load crypto pepper var
        let crypto_pepper = dotenvy::var("CRYPTO_PEPPER")
            .expect("CRYPTO_PEPPER is not set")
            .into();

        // load secret key var
        let secret_key = dotenvy::var("COOKIE_SECRET")
            .expect("COOKIE_SECRET is not set")
            .into();

        // load postgres connection
        let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let connection = PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

        ServerConfig {
            crypto_pepper,
            secret_key,
            connection: Mutex::new(connection),
        }
    }
}

// available globally
pub static SERVER_CONFIG: Lazy<ServerConfig> = Lazy::new(ServerConfig::new);