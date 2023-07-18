use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::serde::json::Json;
use validator::Validate;

use crate::jwt::*;
use crate::models::*;
use crate::schema::users::dsl::*;
use crate::LogsDbConn;

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
pub async fn register(
    conn: LogsDbConn,
    new_user: Json<NewUser>,
) -> Result<Json<User>, NetworkResponse> {
    new_user
        .validate()
        .map_err(|_err| NetworkResponse::BadRequest("Invalid user input".to_string()))?;

    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            return Err(NetworkResponse::InternalServerError(format!(
                "Failed to hash password: {}",
                err
            )))
        }
    };

    let new_user = NewUser {
        username: new_user.username.clone(),
        password: hashed_password.clone(),
    };

    let result = conn
        .run(move |c| {
            diesel::insert_into(users)
                .values(&new_user)
                .get_result::<User>(c)
        })
        .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(NetworkResponse::InternalServerError(format!(
            "Failed to insert new user: {}",
            err
        ))),
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
pub async fn login(
    conn: LogsDbConn,
    user: Json<LoginUser>,
) -> Result<Json<ResponseBody>, NetworkResponse> {
    let token = login_user(conn, user).await?;
    Ok(Json(ResponseBody::AuthToken(token)))
}

pub async fn login_user(
    conn: LogsDbConn,
    login_user: Json<LoginUser>,
) -> Result<String, NetworkResponse> {
    login_user
        .validate()
        .map_err(|_err| NetworkResponse::BadRequest("Invalid user input".to_string()))?;
    let login_user_clone = login_user.clone();
    let result = conn
        .run(move |c| {
            users
                .filter(username.eq(&login_user_clone.username))
                .first::<User>(c)
        })
        .await;

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
                            }
                            Err(err) => {
                                eprintln!("JWT token generation error: {:?}", err);
                                Err(NetworkResponse::InternalServerError(
                                    "Failed to generate JWT token".to_string(),
                                ))
                            }
                        }
                    } else {
                        Err(NetworkResponse::Unauthorized(
                            "Failed to authorize access".to_string(),
                        ))
                    }
                }
                Err(err) => Err(NetworkResponse::InternalServerError(format!(
                    "Failed to verify password: {}",
                    err
                ))),
            }
        }
        Err(err) => Err(NetworkResponse::NotFound(format!(
            "Failed to find user: {}",
            err
        ))),
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
pub async fn profile(
    conn: LogsDbConn,
    key: Result<Jwt, NetworkResponse>,
) -> Result<Json<UserProfile>, NetworkResponse> {
    let key = key?;
    let user_id = key.claims.subject_id;

    match fetch_user_profile(conn, user_id).await {
        Ok(user_profile) => Ok(Json(user_profile)),
        Err(_) => Err(NetworkResponse::NotFound(
            "User profile not found".to_string(),
        )),
    }
}

async fn fetch_user_profile(
    conn: LogsDbConn,
    user_id: i32,
) -> Result<UserProfile, NetworkResponse> {
    use crate::schema::users::dsl::*;

    let user = conn
        .run(move |c| users.filter(id.eq(user_id)).first::<User>(c))
        .await;

    match user {
        Ok(user) => Ok(UserProfile {
            username: user.username,
        }),
        Err(err) => Err(NetworkResponse::NotFound(format!(
            "Failed to find user: {}",
            err
        ))),
    }
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
pub async fn change_password(
    conn: LogsDbConn,
    key: Result<Jwt, NetworkResponse>,
    change_password_request: Json<ChangePasswordRequest>,
) -> Result<NetworkResponse, NetworkResponse> {
    let user_id = key?.claims.subject_id;

    let user = match conn
        .run(move |c| users.filter(id.eq(user_id)).first::<User>(c))
        .await
    {
        Ok(user) => user,
        Err(err) => {
            return Err(NetworkResponse::NotFound(format!(
                "Failed to find user: {}",
                err
            )))
        }
    };

    match verify(&change_password_request.old_password, &user.password) {
        Ok(valid) => {
            if valid {
                let new_password = match hash(&change_password_request.new_password, DEFAULT_COST) {
                    Ok(hash) => hash,
                    Err(err) => {
                        return Err(NetworkResponse::InternalServerError(format!(
                            "Failed to hash password: {}",
                            err
                        )))
                    }
                };
                match conn
                    .run(move |c| {
                        diesel::update(users.filter(id.eq(user_id)))
                            .set(password.eq(new_password))
                            .execute(c)
                    })
                    .await
                {
                    Ok(_) => Ok(NetworkResponse::Ok(
                        "Password successfully changed".to_string(),
                    )),
                    Err(err) => Err(NetworkResponse::InternalServerError(format!(
                        "Failed to update password: {}",
                        err
                    ))),
                }
            } else {
                Err(NetworkResponse::BadRequest(
                    "Incorrect current password".to_string(),
                ))
            }
        }
        Err(err) => Err(NetworkResponse::InternalServerError(format!(
            "Failed to verify old password: {}",
            err
        ))),
    }
}
