
mod auth;
mod schema;
mod shared;
mod swagger;
pub mod files;
pub mod folders;
mod file_store;

pub use auth::service::get_user;

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{get, http::header, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_cors::Cors;


use diesel::{
    r2d2::{self, ConnectionManager},
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared::common::{AppState, Config};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::file_store::FileStore;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[get("/healthcheck")]
async fn health_check(_name: web::Path<String>) -> impl Responder {
    format!("WebServer Status: {}\nDatabase Status {}\n", "Ok", "Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // let env = Env::new().filter("APP_LOG").write_style("APP_LOG_STYLE");
    // env_logger::init_from_env(env);
    env_logger::init();
    
    let config = Config::init();

    // set up database connection pool
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut connection  = pool.get().expect("Failed to get connection from pool");

    connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

    
    //TODO: Support multiple Storage services
    let storage = FileStore::new(std::env::var("FILE_STORE_BASE_PATH").expect("FILE_STORE_BASE_PATH must be set"));
    
    // let result: CountResult = sql_query("SELECT COUNT(*) AS count FROM information_schema.tables WHERE table_name = 'users'")
    //     .get_result::<CountResult>(&mut conn)
    //     .expect("Failed to execute test query");
    
    // let is_db_ready = result.count == 1;
    // if !is_db_ready {
    //     log::error!("Database not initialized. Please run the migrations in production before starting the server.");
    // }

    log::info!("Starting server at: http://localhost:8090");

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8090")
            .allowed_origin("ws://localhost:8090")
            .allowed_methods(vec!["GET", "PUT", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState::new(
                pool.clone(),
                config.clone(),
                // Box::new(storage.clone()),
                storage.clone(),
                std::env::var("PROD_MODE").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false)
            )))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .configure(auth::config)
                    .configure(files::config)
                    .configure(folders::config)
                    // .configure(users::config)
                    // .configure(blocks::config)
                    // .configure(pages::config),
            )
            .service(health_check)
            // .service(web::resource("/ws").to(web_socket))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger::ApiDoc::openapi()),
            )
            // .route("/{filename:.*}", web::get().to(index))
    })
    .workers(2)
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
