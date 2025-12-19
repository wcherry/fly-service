use utoipa::OpenApi;

use crate::auth;
use crate::files;

#[derive(OpenApi)]
#[openapi(
    paths(
    // Auth
        auth::register_user_handler, 
        auth::login_user_handler,
        auth::logout_handler,
    // Tasks
        files::get_file_handler,
        files::get_file_contents_handler,
    ),
    // components(
    //     schemas(
    //         shared::dto::UserDto, 
    //         auth::dto::RegisterUserDto, 
    //         auth::dto::LoginRequestDto, 
    //         auth::dto::LoginResponseDto,
    //         shared::dto::UserProfileDto,
    //     ),
    // ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tags(
        (name = "task_now::api", description = "Task NOW API", external_docs(url = "http://more.about.our.apis", description = "More about our APIs")),
        (name = "Authentication", description = "Authentication related endpoints"),
        (name = "Tasks", description = "Task management endpoints"),
    ),
    external_docs(url = "http://more.about.our.apis", description = "More about our APIs")
)]
pub struct ApiDoc;
