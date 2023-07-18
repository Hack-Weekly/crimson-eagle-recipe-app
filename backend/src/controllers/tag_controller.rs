use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::LogsDbConn;
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
pub async fn tag_list(conn: LogsDbConn) -> Result<Json<Vec<TagDTO>>, Status> {
    match conn.run(|c| tags::table.load::<Tag>(c)).await {
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
pub async fn single_tag(conn: LogsDbConn, tag_slug: String) -> Result<Json<TagDTO>, Status> {
    match conn.run(|c| tags::table
        .filter(tags::slug.eq(tag_slug))
        .first::<Tag>(c)).await
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
pub async fn create_tag(conn: LogsDbConn, tag: Json<TagPostDTO>) -> Result<Json<TagDTO>, Status> {
    match conn.run(|c| diesel::insert_into(tags::table)
        .values(TagDTO::from(tag.into_inner()))
        .get_result::<Tag>(c)).await
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
pub async fn toggle_tag(conn: LogsDbConn, tag_slug: String, recipe_id: i32) -> Result<Json<Vec<TagDTO>>, Status> {
    let tag = match conn.run(|c| tags::table
        .filter(tags::slug.eq(tag_slug))
        .first::<Tag>(c)).await
    {
        Ok(res) => res,
        Err(_) => return Err(Status::NotFound),
    };

    match conn.run(move |c| recipes_tags::table
        .find((tag.id, recipe_id))
        .first::<RecipeTag>(c)).await
    {
        Ok(res) => {
            // remove
            match conn.run(move |c| diesel::delete(&res).execute(c)).await {
                Ok(_) => (),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(diesel::NotFound) => {
            // add
            match conn.run(move |c| diesel::insert_into(recipes_tags::table)
                .values((
                    recipes_tags::recipe_id.eq(recipe_id),
                    recipes_tags::tag_id.eq(tag.id),
                ))
                .execute(c)).await
            {
                Ok(_) => (),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(_) => return Err(Status::InternalServerError),
    }

    match conn.run(move |c| recipes_tags::table
        .filter(recipes_tags::recipe_id.eq(recipe_id))
        .inner_join(tags::table)
        .select(Tag::as_select())
        .load::<Tag>(c)).await
    {
        Ok(t) => Ok(Json(
            t.into_iter().map(TagDTO::from).collect::<Vec<TagDTO>>(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}
