pub mod dto;
pub mod service;

use actix_multipart::Multipart;
use actix_web::{
    get, post, web, Error, HttpResponse,
};

use futures_util::TryStreamExt;
use log::info;

use crate::auth::jwt_auth;
use crate::get_user;
use crate::shared::common::ServiceError;
use crate::shared::common::AppState;
use crate::shared::common::build_full_path;
use crate::shared::dto::{CreateResponseDto, QueryParams};
use service::{get_file, get_file_contents, create_file, get_all_files, update_file};

use dto::{FileDto, CreateFileDto};

///
/// Gets all files for a user
///
#[utoipa::path(
    get,
    tag = "Files",
    path = "/api/files",

    responses(
        (status = 200, description = "Successfully retrieved all files", body = [Vec<FileDto>])
    )
)]
#[get("")]
pub async fn get_all_files_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    query: web::Query<QueryParams>
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;

    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    match get_all_files(&mut conn, user_id, query.into_inner()) {
        Ok(files) => Ok(HttpResponse::Ok().json(files)),
        Err(err) => Err(ServiceError::NotFound(err.to_string()).into()),
    }
}


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

    match get_file(&mut conn, &file_id, user_id) {
        Ok(file) => Ok(HttpResponse::Ok().json(file)),
        Err(err) => Err(ServiceError::NotFound(err.to_string()).into()),
    }
}


///
/// Creates a file
///
#[utoipa::path(
    post,
    tag = "Files",
    path = "/api/files",
    responses(
        (status = 201, description = "Successfully created a file", body = [CreateFileDto])
    )
)]
#[post("")]
pub async fn create_file_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    data: web::Json<CreateFileDto>,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let file = data.into_inner();
    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    match create_file(&mut conn, file, user_id) {
        Ok(uuid) => Ok(HttpResponse::Created().json(CreateResponseDto::ok_with_id(uuid))),
        Err(err) => Err(ServiceError::NotFound(err.to_string()).into()),
    }
}

///
/// Uploads a file
/// 
#[utoipa::path(
    post,
    tag = "Files",
    path = "/api/files/{file_id}upload",
    responses(
        (status = 201, description = "Successfully uploaded a file", body = [FileDto])
    )
)]
#[post("/{file_id}/upload")]
pub async fn upload_file_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
    mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Iterate over the fields in the multipart stream
    if let Some(field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let file_id: String = path.to_string();
        let user_id = jwt.user_id;
        let org_filename = content_disposition.get_filename().unwrap_or("unknown").to_string();
        let file_media_type : String= content_disposition
            .get_filename()
            .map(|_| "application/octet-stream".to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let mut conn = app
            .get_connection()
            .map_err(|err| ServiceError::NotFound(err.to_string()))?;

        let user = get_user(&mut conn, user_id).map_err(|err| ServiceError::NotFound(err.to_string()))?;
        let mut file: FileDto = get_file(&mut conn, &file_id, user_id).map_err(|err| ServiceError::NotFound(err.to_string()))?;

        let full_path = build_full_path(&user.folder_id, &file.folder_id,);

        info!("Saving file: {file_id} to {full_path}");

        let stream: &mut futures_util::stream::IntoStream<actix_multipart::Field> = &mut field.into_stream();

        // upload_file(full_path, &mut stream).await.map_err(|err| ServiceError::BadRequest(err.to_string()))?;

        let storage = app.get_storage_service();
        storage.save_file(&user.folder_id, &file_id, stream).await.map_err(|err| ServiceError::InternalServerError(err.to_string()))?;
        file.media_type = Some(file_media_type);
        file.orginal_filename = Some(org_filename);

        update_file(&mut conn, file, user_id).map_err(|err| ServiceError::InternalServerError(err.to_string()))?;

    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

///
/// Downloads a file
///
#[utoipa::path(
    get,
    tag = "Files",
    path = "/api/files/{file_id}/contents",
    responses(
        (status = 200, description = "Successfully downloaded a file", body = [Vec<u8>])
    )
)]
#[get("/{file_id}/contents")]
pub async fn get_file_contents_handler(
    app: web::Data<AppState>,
    jwt: jwt_auth::JwtMiddleware,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let file_id = path.to_string();
    let user_id = jwt.user_id;

    let mut conn = app
        .get_connection()
        .map_err(|err| ServiceError::NotFound(err.to_string()))?;

    let user = get_user(&mut conn, user_id).map_err(|err| ServiceError::NotFound(err.to_string()))?;
    let file = get_file(&mut conn, &file_id, user_id).map_err(|err| ServiceError::NotFound(err.to_string()))?;
    if file.owner_id != user_id {
        return Err(ServiceError::Unauthorized.into());
    }
    Ok(HttpResponse::Ok()
        .content_type(file.media_type.unwrap_or("application/octet-stream".to_string()))
        .insert_header(("FileName", file.orginal_filename.unwrap_or("Unknown".to_string())))
        .body(get_file_contents(build_full_path(&user.folder_id, &file.folder_id), &file.id)
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?)
    )


}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/files")
            .service(get_all_files_handler)
            .service(get_file_handler)
            .service(get_file_contents_handler)
            .service(create_file_handler)
            .service(upload_file_handler)
            ;

    conf.service(scope);
}


