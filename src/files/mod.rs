pub mod dto;
pub mod service;


use actix_web::{
    get, web, Error, HttpResponse,
};

use crate::auth::jwt_auth;
use crate::shared::common::ServiceError;
use crate::shared::common::AppState;
use service::{get_file, get_file_contents};

use dto::{FileDto};

///
/// Gets a file by it's file id
///
#[utoipa::path(
    get,
    tag = "Files",
    path = "/api/files/{file_id}",

    responses(
        (status = 200, description = "Successfully retrieved a file", body = [FileDto])
    )
)]
#[get("/{file_id}")]
pub async fn get_file_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let file_id: String = path.to_string();
    log::debug!("user_id: {}, file_id: {}", user_id, file_id);

    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    match get_file(&mut conn, file_id, user_id) {
        Ok(task) => Ok(HttpResponse::Ok().json(task)),
        Err(err) => Err(ServiceError::NotFound(err.to_string()).into()),
    }
}

///
/// Gets a file contentsby it's file id
///
#[utoipa::path(
    get,
    tag = "Files",
    path = "/api/files/{file_id}/content",

    responses(
        (status = 200, description = "Successfully retrieved a files contents", body = [String])
    )
)]
#[get("/{file_id}/content")]
pub async fn get_file_contents_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let file_id: String = path.to_string();
    log::debug!("user_id: {}, file_id: {}", user_id, file_id);

    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    let file = get_file(&mut conn, file_id, user_id).map_err(|err| ServiceError::NotFound(err.to_string()))?;
    //TODO: build the complete file path, right now can old get file contents for files in the root folder
    Ok(HttpResponse::Ok().body(get_file_contents(file.folder_id, file.id)
.map_err(|err| ServiceError::NotFound(err.to_string()))?))

}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/files")
            .service(get_file_handler)
            .service(get_file_contents_handler)
            ;

    conf.service(scope);
}


