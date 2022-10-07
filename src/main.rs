mod connection;
mod controller;
mod model;
mod schema;

use actix_web::{web, App, HttpServer};
use connection::connection_to_db;
use controller::get_users;
use diesel::PgConnection;

struct AppData {
    db_conn: std::sync::Arc<std::sync::Mutex<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_connection = connection_to_db();

    let app_data = web::Data::new(AppData {
        db_conn: std::sync::Arc::new(std::sync::Mutex::new(db_connection)),
    });

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(get_users))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
