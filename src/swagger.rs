use utoipa::OpenApi;

use crate::auth;
use crate::files;
use crate::folders;

#[derive(OpenApi)]
#[openapi(
    paths(
    // Auth
        auth::register_user_handler, 
        auth::login_user_handler,
        auth::logout_handler,
    // Files
        files::get_file_handler,
        files::get_file_contents_handler,
        files::create_file_handler,
        files::upload_file_handler,
        files::get_all_files_handler,
    // Folders
        folders::get_all_folders_handler,
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
        (name = "fly::api", description = "Fly API", external_docs(url = "http://more.about.our.apis", description = "More about our APIs")),
        (name = "Authentication", description = "Authentication related endpoints"),
        (name = "Files", description = "File management endpoints"),
    ),
    external_docs(url = "http://more.about.our.apis", description = "More about our APIs")
)]
pub struct ApiDoc;
