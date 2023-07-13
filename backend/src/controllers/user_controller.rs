use diesel::prelude::*;
use rocket::serde::json::Json;
use validator::Validate;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::database;
use crate::models::*;
use crate::schema::users::dsl::*;
use crate::jwt::*;

/// Register a new user
///
/// Create a new user in the database.
#[utoipa::path(
    post,
    path = "/register",
    request_body = NewUser,
    tag = "users",
    responses(
        (status = 200, description = "User registered succesfully", body = User),
        (status = 400, description = "Invalid user input"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/register", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Json<User>, NetworkResponse> {
    new_user.validate().map_err(|_err| NetworkResponse::BadRequest("Invalid user input".to_string()))?;

    let connection = &mut database::establish_connection();
    let hashed_password = match hash(new_user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => return Err(NetworkResponse::InternalServerError(format!("Failed to hash password: {}", err))),
    };

    let new_user = NewUser {
        username: new_user.username,
        password: &hashed_password,
    };

    match diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(connection)
    {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(NetworkResponse::InternalServerError(format!("Failed to insert new user: {}", err))),
    }
}

/// User login
///
/// Authenticate a user and return a JWT token.
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginUser,
    tag = "users",
    responses(
        (status = 200, description = "User logged in succesfully", body = String),
        (status = 400, description = "Invalid user input"),
        (status = 401, description = "Failed to authorize access"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
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
            match verify(&login_user.password, &user.password) {
                Ok(valid) => {
                    if valid {
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
                        Err(NetworkResponse::Unauthorized("Failed to authorize access".to_string()))
                    }
                },
                Err(err) => {
                    Err(NetworkResponse::InternalServerError(format!("Failed to verify password: {}", err)))
                }
            }
        }
        Err(err) => Err(NetworkResponse::NotFound(format!("Failed to find user: {}", err))),
    }
}

/// Fetch User Profile
///
/// Returns the user profile information.
#[utoipa::path(
    get,
    path = "/profile",
    tag = "users",
    responses(
        (status = 200, description = "User profile found succesfully", body = UserProfile),
        (status = 404, description = "User profile not found"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
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

/// Change User Password
///
/// Updates the password of the current user.
#[utoipa::path(
    put,
    path = "/profile/change_password",
    request_body = ChangePasswordRequest,
    tag = "users",
    responses(
        (status = 200, description = "Password successfully changed"),
        (status = 400, description = "Incorrect current password"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[put("/profile/change_password", data = "<change_password_request>")]
pub fn change_password(key: Result<Jwt, NetworkResponse>, change_password_request: Json<ChangePasswordRequest>) -> Result<NetworkResponse, NetworkResponse> {
    let user_id = key?.claims.subject_id;

    let connection = &mut database::establish_connection();
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .map_err(|err| NetworkResponse::NotFound(format!("Failed to find user: {}", err)))?;

    match verify(&change_password_request.old_password, &user.password) {
        Ok(valid) => {
            if valid {
                let new_password = match hash(&change_password_request.new_password, DEFAULT_COST) {
                    Ok(hash) => hash,
                    Err(err) => return Err(NetworkResponse::InternalServerError(format!("Failed to hash password: {}", err))),
                };
                diesel::update(users.filter(id.eq(user_id)))
                    .set(password.eq(new_password))
                    .execute(connection)
                    .map_err(|err| NetworkResponse::InternalServerError(format!("Failed to update password: {}", err)))?;
                Ok(NetworkResponse::Ok("Password successfully changed".to_string()))
            } else {
                Err(NetworkResponse::BadRequest("Incorrect current password".to_string()))
            }
        },
        Err(err) => Err(NetworkResponse::InternalServerError(format!("Failed to verify old password: {}", err))),
    }
}