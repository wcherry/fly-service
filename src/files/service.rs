use std::fs::File;
use std::io::Write;

use crate::shared::{common::{DbError, ServiceError}, dto::QueryParams};
use super::dto::{FileDto, CreateFileDto};
use actix_multipart::Field;
use diesel::prelude::*;
use uuid::Uuid;

use futures_util::TryStreamExt;

use crate::schema::files::dsl;


pub fn create_file(conn: &mut SqliteConnection, file: CreateFileDto, owner_id: i32) -> Result<String, DbError> {
    let uuid = Uuid::new_v4().to_string();
    let file = FileDto {
        id: uuid,
        title: file.title,
        owner_id,
        access_level: file.access_level,
        media_type: file.media_type,
        description: file.description,
        folder_id: file.folder_id,
        orginal_filename: None,
        created_by: owner_id,
        updated_by: owner_id,
        created_at: None,
        updated_at: None,
        active: true,
    };

    diesel::insert_into(dsl::files)
        .values(&file)
        .execute(conn)?;
    Ok(file.id)
}


pub fn update_file(conn: &mut SqliteConnection, file: FileDto, owner_id: i32) -> Result<usize, DbError> {
    // let file = FileDto {
    //     id: uuid,
    //     title: file.title,
    //     owner_id,
    //     access_level: file.access_level,
    //     media_type: file.media_type,
    //     description: file.description,
    //     folder_id: file.folder_id,
    //     orginal_filename: None,
    //     created_by: owner_id,
    //     updated_by: owner_id,
    //     created_at: None,
    //     updated_at: None,
    //     active: true,
    // };

    Ok(diesel::update(dsl::files.filter(dsl::id.eq(file.id).and(dsl::owner_id.eq(owner_id))))
        .set((
            dsl::title.eq(file.title),
            dsl::access_level.eq(file.access_level),
            dsl::media_type.eq(file.media_type),
            dsl::description.eq(file.description),
            dsl::folder_id.eq(file.folder_id),
            dsl::updated_by.eq(owner_id),
            dsl::updated_at.eq(chrono::Local::now().naive_local()),
            dsl::orginal_filename.eq(file.orginal_filename),
            dsl::active.eq(file.active)))
        .execute(conn)?)
}

#[allow(dead_code)]
pub async fn upload_file(full_path: String, f: &mut Field) -> Result<(), ServiceError> {
    let mut file = File::create(full_path).expect("Failed to create file");

    // Field in turn is a stream of Bytes object
    while let Some(chunk) = f.try_next().await.map_err(|err| ServiceError::BadRequest(format!("{err}")))? {
        file.write_all(&chunk).map_err(|err| ServiceError::BadRequest(format!("{err}")))?;
    }

    Ok(())
}

pub fn get_all_files(conn: &mut SqliteConnection, user_id: i32, params: QueryParams) -> Result<Vec<FileDto>, DbError> {
    let mut query = crate::schema::files::table.into_boxed::<>();
    query = query.filter(dsl::owner_id.eq(user_id));
    
    if let Some(q_folder_id) = params.folder_id {
        log::info!("Filtering for folder {}",q_folder_id);
        query = query.filter(dsl::folder_id.eq(q_folder_id));
    }

    Ok(query.get_results::<FileDto>(conn)?)
}


pub fn get_file(conn: &mut SqliteConnection, file_id: &String,  user_id: i32) -> Result<FileDto, DbError> {
    let file = dsl::files
        .filter(dsl::id.eq(&file_id))
        .filter(dsl::owner_id.eq(user_id))
        .first::<FileDto>(conn);
    log::debug!("Loaded file_id: {} for user_id: {}: {}", file_id, user_id, match &file {
        Ok(_) => "Ok",
        Err(_) => "Error",
    });
    match file {
        Ok(file) => Ok(file),
        Err(err) => {
            let msg = format!("Error loading file file_id: {}, user_id: {}, Error: {}", file_id, user_id, err);
            log::info!("{}", msg);
            Err(msg.into())
        }
    }
}

pub fn get_file_contents(path: String, file_id: &str) -> Result<String, DbError> {
    use std::fs::File;
    use std::io::Read;

    let full_path = format!("{}/{}", path, file_id);
    let mut file = File::open(full_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(contents)
}