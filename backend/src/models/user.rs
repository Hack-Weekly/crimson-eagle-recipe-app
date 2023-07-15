use crate::schema::users;
use diesel::prelude::*;
use rocket::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use validator::Validate;
use validator::ValidationError;
use lazy_static::lazy_static;
use regex::Regex;
use jsonwebtoken::errors::Error;
use crate::jwt::decode_jwt;

#[derive(FromForm, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

lazy_static! {
    // Username must be at least 3 characters long and can only contain alphanumeric characters
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{3,}$").unwrap();
}
// Password must be at least 6 characters long and contain at least one letter and one number. Can also contain special characters
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_special_char = password.chars().any(|c| "!@#$%^&*".contains(c));

    if has_letter && has_number && (has_special_char || password.chars().all(|c| c.is_alphanumeric())) {
        Ok(())
    } else {
        Err(ValidationError::new("Password requirements not met"))
    }
}

#[derive(FromForm, Insertable, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(regex(path = "USERNAME_REGEX"))]
    pub username: String,
    #[validate(length(min=6), custom(function = "validate_password"))]
    pub password: String,
}

#[derive(FromForm, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub username: String,
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 200)]
    Ok(String),
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 500)]
    InternalServerError(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    pub exp: usize
}

#[derive(Debug)]
pub struct Jwt {
    pub claims: Claims
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response { 
                    body: ResponseBody::Message(
                        String::from("Error validating Jwt token - No token provided")
                    )
                };

                Outcome::Failure((
                    Status::Unauthorized, 
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                )) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(Jwt {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { 
                            body: ResponseBody::Message(
                                format!("Error validating Jwt token - Expired Token")
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating Jwt token - Invalid Token")
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    _ => {
                        let response = Response { 
                            body: ResponseBody::Message(
                                format!("Error validating Jwt token - {}", err)
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized, 
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    }
                }
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}