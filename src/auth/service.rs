use crate::shared::common::{DbError, create_file_folder};
use crate::shared::dto::{NewUserDto, UserDto, CreateUser, User};
use argon2::{PasswordHash, PasswordVerifier};
use diesel::insert_into;
use diesel::{prelude::*};
use uuid::Uuid;

use crate::shared::common::Connection;
use crate::schema::{users};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn get_user(conn: &mut SqliteConnection, user_id: i32) -> Result<UserDto, DbError> {
    let user = users::dsl::users
        .filter(users::id.eq(user_id))
        .select((users::id, users::username, users::email_address, users::folder_id,users::active))
        .first::<UserDto>(conn)?;    
    Ok(user)
}

pub fn find_user_by_username_and_password(conn: &mut Connection, username: String, password: String) -> Result<UserDto, DbError> {
    let user = users::dsl::users
        .filter(users::username.eq(username))
        .select((users::id, users::username, users::password, users::email_address, users::folder_id, users::active))
        .first::<User>(conn)?;    

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok_and(|_| true);

    if !is_valid {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, 
            serde_json::json!({
                "status": "fail",
                "message": "Invalid email or password"
            }).to_string()))
        )
    }
        
    Ok(UserDto {
        id: user.id,
        username: user.username,
        email_address: user.email_address,
        folder_id: user.folder_id,
        active: user.active,
    })
}

pub fn is_exists(conn: &mut Connection, username: String) -> Result<bool, DbError> {
    let exists: i64 = users::dsl::users.filter(users::username.eq(username)).count().get_result(conn)?; // Result<i64, Error>
    Ok(exists == 1)
}

pub fn create_user(
    conn: &mut Connection,
    new_user : NewUserDto,
) -> Result<bool, DbError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(new_user.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let uuid = Uuid::new_v4().to_string();

    let user = CreateUser {
        id: None,
        username: new_user.username,
        password: hashed_password,
        email_address: new_user.email_address,
        folder_id: uuid.to_string(),
        active: false,
    };

    // Create user object
    let num_records = insert_into(users::dsl::users).values(user).execute(conn)?;

    create_file_folder(uuid)?;

    Ok(num_records == 1)
}
