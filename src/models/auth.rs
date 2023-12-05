use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct NewUser {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
