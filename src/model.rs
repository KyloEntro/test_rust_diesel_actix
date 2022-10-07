use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub comment: Option<String>,
    pub added: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub comment: Option<String>,
    pub added: NaiveDateTime,
}
