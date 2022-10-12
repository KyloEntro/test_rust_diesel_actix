use std::sync::PoisonError;

use crate::model::{NewUser, User};
use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;

#[get("/users")]
pub async fn get_users(data: web::Data<AppData>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let db_conn = &mut *data.db_conn.lock().unwrap_or_else(PoisonError::into_inner);
    let users_data = users.load::<User>(db_conn).expect("Error to load users");
    HttpResponse::Ok().body(serde_json::to_string(&users_data).unwrap())
}

#[derive(Deserialize)]
pub struct UserPayload {
    username: String,
    password: String,
    comment: Option<String>,
}

#[post("/user")]
pub async fn create_user(
    payload: web::Json<UserPayload>,
    data: web::Data<AppData>,
) -> impl Responder {
    use crate::schema::users;

    let db_conn = &mut *data.db_conn.lock().unwrap_or_else(PoisonError::into_inner);

    let hash = bcrypt::hash(payload.password.to_owned(), 4).unwrap();

    let new_user = NewUser {
        username: payload.username.clone(),
        password_hash: hash,
        comment: payload.comment.clone(),
        added: Utc::now().naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(db_conn)
        .expect("Error saving new user");

    HttpResponse::Created()
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[post("/user/login")]
pub async fn login_user(
    payload: web::Json<LoginPayload>,
    data: web::Data<AppData>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let db_conn = &mut *data.db_conn.lock().unwrap_or_else(PoisonError::into_inner);

    let user_found = users
        .filter(username.eq(&payload.username))
        .first::<User>(db_conn)
        .optional()
        .unwrap();

    if user_found.is_none() {
        return HttpResponse::NotFound();
    }

    let user = user_found.unwrap();
    if !bcrypt::verify(&payload.password, &user.password_hash).unwrap() {
        return HttpResponse::Unauthorized();
    }

    HttpResponse::Ok()
}
