use crate::controllers::*;
use crate::models::*;
use rocket::http::Status;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Crimson Eagle's Recipe App"
    ),
    paths(
        bookmark_controller::bookmarked_list,
        bookmark_controller::toggle_bookmark,
        recipe_controller::recipe,
        recipe_controller::search,
        recipe_controller::single_recipe,
        recipe_create_controller::create_recipe,
        recipe_update_controller::update_recipe,
        recipe_controller::delete,
        tag_controller::tag_list,
        tag_controller::single_tag,
        tag_controller::create_tag,
        tag_controller::toggle_tag,
        user_controller::login,
        user_controller::register,
        user_controller::profile,
        user_controller::change_password
    ),
    components(
        schemas(RecipeResultDTO, RecipesInput, RecipePutDTO, PaginatedResult<RecipeResultDTO>),
    ),
    tags(
        (name = "recipes", description = "Recipes endpoints."),
        (name = "auth", description = "Authentication endpoints."),
    ),
    servers(
        (url = "http://127.0.0.1:8000", description = "Local development"),
        (url = "https://crimson-eagles-recipe-app.onrender.com", description = "Deployed")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[get("/api-docs/openapi.json")]
pub fn serve_api_doc() -> Result<String, Status> {
    let openapi = ApiDoc::openapi().to_pretty_json().unwrap();

    Ok(openapi)
}
