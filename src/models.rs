/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
  pub id: i32,
  pub first_name: String,
  pub last_name: Option<String>,
  pub email: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'x> {
  pub first_name: &'x str,
  pub last_name: Option<&'x str>,
}
