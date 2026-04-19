use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    Google,
    Apple,
}

impl std::fmt::Display for AuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthProvider::Google => write!(f, "google"),
            AuthProvider::Apple => write!(f, "apple"),
        }
    }
}

impl std::str::FromStr for AuthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(AuthProvider::Google),
            "apple" => Ok(AuthProvider::Apple),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tz: String,
    pub email: String,
    pub name: String,
    pub provider: String,
    pub provider_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new( tz: String, email: String, name: String, provider: AuthProvider, provider_id: String,) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tz, 
            email,
            name,
            provider: provider.to_string(),
            provider_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub tz: String, 
    pub provider: String,
    pub provider_id: String,
}

impl From<CreateUser> for User {
    fn from(input: CreateUser) -> Self {
        let now = Utc::now();

        Self { 
            id: Uuid::new_v4(),
            tz: input.tz,
            email: input.email,
            name: input.name,
            provider: input.provider,
            provider_id: input.provider_id,
            created_at: now,
            updated_at: now
        }
    }
}
