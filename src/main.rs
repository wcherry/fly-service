
mod auth;
mod schema;
mod shared;
mod swagger;
pub mod files;

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

#[get("/healthcheck")]
async fn health_check(_name: web::Path<String>) -> impl Responder {
    format!("WebServer Status: {}\nDatabase Status {}\n", "Ok", "Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    let config = Config::init();

    // set up database connection pool
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let _ = pool.get().expect("Failed to get connection from pool");
    
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
                true,                // is_db_ready,
            )))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .configure(auth::config)
                    .configure(files::config)
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
