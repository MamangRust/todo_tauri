#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub run_migrations: bool,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().expect("Failed to read .env file");

        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = dotenv::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = dotenv::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let run_migrations_str = dotenv::var("RUN_MIGRATIONS").expect("RUN_MIGRATIONS must be set");
        let port_str = dotenv::var("PORT").expect("PORT must be set");

        let run_migrations = match run_migrations_str.as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("RUN_MIGRATIONS must be either 'true' or 'false'"),
        };

        let port = port_str.parse().expect("Invalid value for PORT");

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse().expect("Invalid value for JWT_MAXAGE"),
            run_migrations,
            port,
        }
    }
}
