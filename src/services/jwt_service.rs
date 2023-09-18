use std::env;
use std::ops::Add;

use anyhow::{anyhow, Context, Result};
use chrono::{Days, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use tokio::task;

use crate::models::user::{Claims, User};

pub async fn create(user: &User) -> Result<String> {
    let user_id = user.id;

    task::spawn_blocking(move || {
        let secret = env::var("JWT_SECRET_KEY").map_err(|e| anyhow!(e))?;

        let exp = Utc::now().add(Days::new(30)).timestamp() as usize;
        let claims = Claims {
            sub: user_id,
            exp: exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| anyhow!(e))?;

        Ok(token)
    })
    .await
    .context("failure in create()")?
}

pub async fn verify(token: &str) -> Result<Claims> {
    let t = token.to_string();
    task::spawn_blocking(move || {
        let secret = env::var("JWT_SECRET_KEY").map_err(|e| anyhow!(e))?;
        let token_data = decode::<Claims>(
            &t,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|e| anyhow!(e))?;

        Ok(token_data.claims)
    })
    .await?
}
