use crate::shared::common::{DbError};
use super::dto::{FolderDto};
use diesel::prelude::*;

pub fn get_all_folders_in_folder(conn: &mut SqliteConnection, user_id: i32, folder_id: String) -> Result<Vec<FolderDto>, DbError> {
    use crate::schema::file_folders::dsl::*;

    let results = file_folders
        .filter(owner_id.eq(user_id))
        .filter(parent_folder_id.eq(folder_id))
        .load::<FolderDto>(conn)?;

    Ok(results)
}   



