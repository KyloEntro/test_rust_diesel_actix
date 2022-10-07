mod connection;
mod controller;
mod model;
mod schema;

use actix_web::{web, App, HttpServer};
use connection::connection_to_db;
use controller::{create_user, get_users};
use diesel::PgConnection;

pub struct AppData {
    db_conn: std::sync::Arc<std::sync::Mutex<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_connection = connection_to_db();

    let app_data = web::Data::new(AppData {
        db_conn: std::sync::Arc::new(std::sync::Mutex::new(db_connection)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_users)
            .service(create_user)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
