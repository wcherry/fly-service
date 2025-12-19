use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Option<i32>,
    pub username: String,
    pub email_address: String,
    // Has the user activated their account via email confirmation?
    pub active: bool,   
}

#[derive(Debug, Clone)]
#[derive(Selectable, Queryable, Insertable, ToSchema)]
#[diesel(table_name = users)]
// #[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub email_address: String,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewUserDto {
    pub username: String,
    pub password: String,
    pub email_address: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileDto {
    pub id: i32,
    pub name: String,
    pub email_address: String,
    pub role: String,
    pub profile_id: Option<i32>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub default_page_id: Option<String>,
    pub page_version_id: Option<String>,
    pub company_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: i32,
    pub updated_by: i32,
    pub active: bool,
}

// impl std::convert::From<UserProfile> for UserProfileDto {
//     fn from(user: UserProfile) -> Self {
//         UserProfileDto {
//             id: user.id,
//             name: user.name,
//             email_address: user.email_address,
//             role: user.role,
//             profile_id: user.profile_id,
//             avatar_url: user.avatar_url,
//             bio: user.bio,
//             default_page_id: user.default_page_id,
//             page_version_id: user.page_version_id,
//             company_id: user.company_id,
//             created_at: user.created_at,
//             updated_at: user.updated_at,
//             created_by: user.created_by,
//             updated_by: user.updated_by,
//             active: user.active,
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[allow(dead_code)]
impl CreateResponse {
    pub fn ok_msg(message: String) -> Self {
        CreateResponse { success: true, message: Some(message) }
    }

    pub fn ok() -> Self {
        CreateResponse { success: true, message: None }
    }

    pub fn err(message: String) -> Self {
        CreateResponse { success: false, message: Some(message) }
    }

}