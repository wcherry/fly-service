use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schema::{file_folders};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = file_folders)]
#[serde(rename_all = "camelCase")]
pub struct FolderDto {
    // The unique identifier for the folder
    pub id: String,
    // The ID of the user who owns the folder
    pub owner_id: i32,
    pub parent_folder_id: String,
    pub title: String,
    // Additional description of the folder
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderDto {
    pub title: String,
    pub parent_folder_id: String,
    pub description: Option<String>,
}

