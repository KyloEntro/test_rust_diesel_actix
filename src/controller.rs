use crate::model::User;
use crate::AppData;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use serde_json;

#[get("/users")]
pub async fn get_users(data: web::Data<AppData>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let db_conn = &mut *data.db_conn.lock().unwrap();
    let users_data = users.load::<User>(db_conn).expect("Error to load users");
    HttpResponse::Ok().body(serde_json::to_string(&users_data).unwrap())
}
