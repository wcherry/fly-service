// use std::{sync::{Arc, Mutex}};

use std::io::Error;

use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::{
    prelude::*, r2d2::{self, ConnectionManager, PooledConnection}
};

use async_trait::async_trait;

use crate::file_store::FileStore;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

// Format the response as JSON instead of the default text
// actix_web::error::ErrorBadRequest(err)
// ref: https://stackoverflow.com/questions/64291039/how-to-return-the-error-description-in-a-invalid-json-request-body-to-the-client
// also see the following url for solution for all errors:
// ref: https://stackoverflow.com/questions/57878917/why-does-an-actix-web-service-send-text-plain-instead-of-json-for-an-auth-error
// actix_web::error::InternalError::from_response(
//     "",
//     HttpResponse::BadRequest()
//         .content_type("application/json")
//         .body(format!(r#"{{"error":"{}"}}"#, err)),
// )

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(r#"{{"error":"Internal Server Error - {}"}}"#, _0)]
    InternalServerError(String),

    #[display(r#"{{"error":"{}"}}"#, _0)]
    BadRequest(String),

    #[display(r#"{{"error":"Unauthorized"}}"#)]
    Unauthorized,

    #[display(r#"{{"error":"Object '{}' not Found"}}"#, _0)]
    NotFound(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError(ref _message) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(self.to_string()),
            ServiceError::BadRequest(ref _message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .body(self.to_string()),
            ServiceError::Unauthorized => HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(self.to_string()),
            ServiceError::NotFound(ref _message) => HttpResponse::NotFound()
                .content_type("application/json")
                .body(self.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub _jwt_expires_in: String,
    pub _jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            database_url,
            jwt_secret,
            _jwt_expires_in: jwt_expires_in,
            _jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}

// #[derive(QueryableByName)]
// pub struct CountResult {
//     #[sql_type = "diesel::sql_types::BigInt"]
//     pub count: i64,
// }

pub struct AppState {
    pool: DbPool,
    config: Config,
    // storage: Box<dyn StorageService>,
    storage: FileStore,
    prod_mode: bool,
}

impl AppState {
    pub fn new(pool: DbPool, config: Config, storage: FileStore, prod_mode: bool) -> AppState {
        AppState {
            pool,
            config,
            storage,
            prod_mode,
        }
    }
    // pub fn set_init_completed(&self, completed: bool) {
    //     self.init_completed.lock().unwrap().clone_from(&completed);
    // }

    // pub fn is_init_completed(&self) -> bool {
    //     *self.init_completed.lock().unwrap()
    // }

    pub fn get_connection(&self) -> Result<Connection, DbError> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                format!("No connection available: {}", e),
            ))),
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn is_prod_mode(&self) -> bool {
        self.prod_mode
    }

    pub fn get_storage_service(&self) -> &FileStore { //&Box<dyn StorageService> {
        &self.storage
    }    
}

pub fn build_full_path(user_folder: &str, file_folder: &str) -> String {
    match user_folder == file_folder {
        true => user_folder.to_string(),
        false => format!("{}/{}", user_folder, file_folder),
    }
}

#[allow(dead_code)]
#[async_trait]
pub trait StorageService {
    async fn save_file(&mut self, path: String, name: String, input: &mut futures_util::stream::IntoStream<actix_multipart::Field>) -> Result<(), Error>;
    fn retrieve_file(&self, path: String, name: String) -> Result<Vec<u8>, Error>;
    fn create_folder(&self, path: String) -> Result<(), Error>;
    fn list_file_names(&self, path: String) -> Result<Vec<String>, Error>;
    fn list_folder_names(&self, path: String) -> Result<Vec<String>, Error>;
}