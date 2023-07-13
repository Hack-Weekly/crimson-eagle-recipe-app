use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;
use crate::schema::*;

/// List of tags
///
/// Get all tags from the database.
#[utoipa::path(
    get,
    path = "/tags",
    tag = "recipes",
    responses(
        (status = 200, description = "Tags found succesfully", body = [Vec<TagDTO>]),
        (status = 500, description = "Error loading tags"),
    )
)]
#[get("/tags")]
pub fn tag_list() -> Result<Json<Vec<TagDTO>>, Status> {
    let connection = &mut database::establish_connection();
    match tags::table.load::<Tag>(connection) {
        Ok(t) => Ok(Json(
            t.into_iter().map(TagDTO::from).collect::<Vec<TagDTO>>(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Find tag
///
/// Get tag by id from the database.
#[utoipa::path(
    get,
    path = "/tags/{tag_slug}",
    tag = "recipes",
    responses(
        (status = 200, description = "Tag found succesfully", body = TagDTO),
        (status = NOT_FOUND, description = "Tag was not found"),
    ),
    params(
        ("tag_slug" = String, description = "Tag slug", example = "gluten-free"),
    )
)]
#[get("/tags/<tag_slug>")]
pub fn single_tag(tag_slug: String) -> Result<Json<TagDTO>, Status> {
    let connection = &mut database::establish_connection();

    match tags::table
        .filter(tags::slug.eq(tag_slug))
        .first::<Tag>(connection)
    {
        Ok(res) => Ok(Json(TagDTO::from(res))),
        Err(_) => Err(Status::NotFound),
    }
}

/// Add tag
///
/// Create new tag in the database.
#[utoipa::path(
    post,
    path = "/tags",
    request_body = TagPostDTO,
    tag = "recipes",
    responses(
        (status = 201, description = "Tag created succesfully", body = TagDTO),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[post("/tags", data = "<tag>")]
pub fn create_tag(tag: Json<TagPostDTO>) -> Result<Json<TagDTO>, Status> {
    let connection = &mut database::establish_connection();

    match diesel::insert_into(tags::table)
        .values(TagDTO::from(tag.into_inner()))
        .get_result::<Tag>(connection)
    {
        Ok(t) => {
            Ok(Json(TagDTO::from(t)))
            /* let tag_url = uri!(single_tag: t.slug);
            Created(tag_url, Ok(Json(TagDTO::from(t)))) */
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Toggle tag on a recipe
///
/// Set or unset tag relationship with a recipe.
/// Returns recipe's tag list.
#[utoipa::path(
    put,
    path = "/tags/{tag_slug}/{recipe_id}",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipe-tag relationship handled succesfully", body = Vec<TagDTO>, example = json!(["vegan", "gluten free"])),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("tag_slug" = String, description = "Tag slug", example = "gluten-free"),
        ("recipe_id" = i32, description = "Recipe id", example = 2),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[put("/tags/<tag_slug>/<recipe_id>")]
pub fn toggle_tag(tag_slug: String, recipe_id: i32) -> Result<Json<Vec<TagDTO>>, Status> {
    let connection = &mut database::establish_connection();

    let tag = match tags::table
        .filter(tags::slug.eq(tag_slug))
        .first::<Tag>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(Status::NotFound),
    };

    match recipes_tags::table
        .find((tag.id, recipe_id))
        .first::<RecipeTag>(connection)
    {
        Ok(res) => {
            // remove
            match diesel::delete(&res).execute(connection) {
                Ok(_) => (),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(diesel::NotFound) => {
            // add
            match diesel::insert_into(recipes_tags::table)
                .values((
                    recipes_tags::recipe_id.eq(recipe_id),
                    recipes_tags::tag_id.eq(tag.id),
                ))
                .execute(connection)
            {
                Ok(_) => (),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(_) => return Err(Status::InternalServerError),
    }

    match recipes_tags::table
        .filter(recipes_tags::recipe_id.eq(recipe_id))
        .inner_join(tags::table)
        .select(Tag::as_select())
        .load::<Tag>(connection)
    {
        Ok(t) => Ok(Json(
            t.into_iter().map(TagDTO::from).collect::<Vec<TagDTO>>(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}
