use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use validator::Validate;

use crate::database;
use crate::models::*;
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify, DEFAULT_COST};

#[post("/register", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    new_user.validate().map_err(|_err| Status::BadRequest)?;

    let connection = &mut database::establish_connection();
    let hashed_password = hash(new_user.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: &new_user.username,
        password: &hashed_password,
    };

    match diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(connection)
    {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>) -> Result<Json<User>, Status> {
    login_user.validate().map_err(|_err| Status::BadRequest)?;

    let connection = &mut database::establish_connection();
    let result = users
        .filter(username.eq(&login_user.username))
        .first::<User>(connection);

    match result {
        Ok(user) => {
            if verify(&login_user.password, &user.password).unwrap() {
                Ok(Json(user))
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}
