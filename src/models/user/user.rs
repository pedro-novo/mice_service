
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}