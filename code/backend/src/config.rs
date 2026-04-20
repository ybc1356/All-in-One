use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub google: OAuthConfig,
    pub apple: AppleConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
    pub refresh_expiration_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppleConfig {
    pub team_id: String,
    pub service_id: String,
    pub key_id: String,
    pub private_key_path: String,
    pub redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let database = DatabaseConfig {
            url: std::env::var("DATABASE_URL")?,
        };

        let jwt = JwtConfig {
            secret: std::env::var("JWT_SECRET")?,
            expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")?.parse()?,
            refresh_expiration_days: std::env::var("JWT_REFRESH_EXPIRATION_DAYS")?.parse()?,
        };

        let google = OAuthConfig {
            client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
            redirect_uri: std::env::var("GOOGLE_REDIRECT_URI")?,
        };

        let apple = AppleConfig {
            team_id: std::env::var("APPLE_TEAM_ID")?,
            service_id: std::env::var("APPLE_SERVICE_ID")?,
            key_id: std::env::var("APPLE_KEY_ID")?,
            private_key_path: std::env::var("APPLE_PRIVATE_KEY_PATH")?,
            redirect_uri: std::env::var("APPLE_REDIRECT_URI")?,
        };

        Ok(Config {
            database,
            jwt,
            google,
            apple,
        })
    }
}
