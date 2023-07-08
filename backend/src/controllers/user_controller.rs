use diesel::prelude::*;
use rocket::serde::json::Json;
use validator::Validate;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::database;
use crate::models::*;
use crate::schema::users::dsl::*;
use crate::jwt::*;

#[post("/register", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Json<User>, NetworkResponse> {
    new_user.validate().map_err(|_err| NetworkResponse::BadRequest("Invalid user input".to_string()))?;

    let connection = &mut database::establish_connection();
    let hashed_password = hash(new_user.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: new_user.username,
        password: &hashed_password,
    };

    match diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(connection)
    {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(NetworkResponse::InternalServerError("Failed to insert new user".to_string())),
    }
}

#[post("/login", data = "<user>")]
pub fn login(user: Json<LoginUser>) -> Result<String, NetworkResponse> {
    let token = login_user(user)?;

    let response = Response { body: ResponseBody::AuthToken(token) };

    Ok(serde_json::to_string(&response).unwrap())
}

pub fn login_user(login_user: Json<LoginUser>) -> Result<String, NetworkResponse> {
    login_user.validate().map_err(|_err| NetworkResponse::BadRequest("Invalid user input".to_string()))?;

    let connection = &mut database::establish_connection();
    let result = users
        .filter(username.eq(&login_user.username))
        .first::<User>(connection);

    match result {
        Ok(user) => {
            if verify(&login_user.password, &user.password).unwrap() {
                // Generate JWT token
                match create_jwt(user.id) {
                    Ok(token) => {
                        println!("Generated token: {}", token);
                        Ok(token)
                    },
                    Err(err) => {
                        eprintln!("JWT token generation error: {:?}", err);
                        Err(NetworkResponse::InternalServerError("Failed to generate JWT token".to_string()))
                    }
                }
            } else {
                Err(NetworkResponse::Unauthorized("Unauthorized access".to_string()))
            }
        }
        Err(_) => Err(NetworkResponse::NotFound("User not found".to_string())),
    }
}

#[get("/profile")]
pub fn profile(key: Result<Jwt, NetworkResponse>) -> Result<Json<UserProfile>, NetworkResponse> {
    let key = key?;
    let user_id = key.claims.subject_id;
    
    match fetch_user_profile(user_id) {
        Ok(user_profile) => Ok(Json(user_profile)),
        Err(_) => Err(NetworkResponse::NotFound("User profile not found".to_string())),
    }
}

fn fetch_user_profile(user_id: i32) -> Result<UserProfile, NetworkResponse> {
    use crate::schema::users::dsl::*;

    let connection = &mut database::establish_connection();

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .map_err(|_| NetworkResponse::NotFound("User not found".to_string()))?;

    Ok(UserProfile {
        username: user.username,
    })
}
