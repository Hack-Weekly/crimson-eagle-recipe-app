use chrono::Utc;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Algorithm, Header, Validation};
use jsonwebtoken::errors::{Error, ErrorKind};
use std::env; 
use crate::models::*;

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); 

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(6))
        .expect("Invalid timestamp")
        .timestamp();
    
    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    }; 

    let header = Header::new(Algorithm::HS512);
    
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}