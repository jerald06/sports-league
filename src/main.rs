use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
mod entity;

mod service;
use crate::service::services::{
    delete_ground_by_name, get_all_grounds, get_ground_details, update_ground_details,
};
use service::services::add_ground_details;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create MySQL pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_ground_details)
            .service(get_ground_details)
            .service(update_ground_details)
            .service(get_all_grounds)
            .service(delete_ground_by_name)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Failed to start the server: {:?}", e);
        e
    })
}
