use crate::shared::common::DbError;
use super::dto::{FileDto};
use diesel::prelude::*;

use crate::schema::files;



pub fn get_file(conn: &mut SqliteConnection, file_id: String,  user_id: i32) -> Result<FileDto, DbError> {
    let task = files::dsl::files
        .filter(files::id.eq(&file_id))
        .filter(files::owner_id.eq(user_id))
        .first::<FileDto>(conn);
    log::debug!("Loaded task_id: {} for user_id: {}: {}", file_id, user_id, match &task {
        Ok(_) => "Ok",
        Err(_) => "Error",
    });
    if task.is_err() {
        let msg = format!("Error loading file file_id: {}, user_id: {}", file_id, user_id);
        log::info!("{}", msg);
        Err(msg.into())
    } else {    
        Ok(task.unwrap())
    }
}

pub fn get_file_contents(file_path: String, file_name: String) -> Result<String, DbError> {
    use std::fs::File;
    use std::io::Read;

    let mut file_path = file_path;
    file_path.push('/');
    file_path.push_str(&file_name);

    let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(contents)
}