pub mod dto;
pub mod service;


use actix_web::{
    get, web, Error, HttpResponse,
};

use log::info;

use crate::auth::jwt_auth;
use crate::shared::common::ServiceError;
use crate::shared::common::AppState;
use service::{get_all_folders_in_folder};

use dto::{FolderDto};

///
/// Gets all folders in a user's folder
///
#[utoipa::path(
    get,
    tag = "Folders",
    path = "/api/folders/{folder_id}/contents",

    responses(
        (status = 200, description = "Successfully retrieved all folders in folder", body = [Vec<FolderDto>])
    )
)]
#[get("/{folder_id}/contents")]
pub async fn get_all_folders_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let folder_id: String = path.to_string();

    info!("Getting all folders in folder: {} for user: {}", folder_id, user_id);

    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    match get_all_folders_in_folder(&mut conn, user_id, folder_id) {
        Ok(folders) => Ok(HttpResponse::Ok().json(folders)),
        Err(err) => Err(ServiceError::NotFound(err.to_string()).into()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/folders")
            .service(get_all_folders_handler)
            // .service(add_move_to_folder_handler)
            ;

    conf.service(scope);
}


