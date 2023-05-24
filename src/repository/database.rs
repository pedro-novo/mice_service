use std::fmt::Error;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::mouse::Mouse;
use crate::repository::schema::mice::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_mice(&self) -> Vec<Mouse> {
        mice
            .load::<Mouse>(&mut self.pool.get().unwrap())
            .expect("Error loading all mice.")
    }

    pub fn create_mouse(&self, mouse: Mouse) -> Result<Mouse, Error> {
        let mouse = Mouse {
            id: uuid::Uuid::new_v4().to_string(),
            ..mouse
        };
        diesel::insert_into(mice)
            .values(&mouse)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new mouse.");
        Ok(mouse)
    }

    pub fn get_mouse_by_id(&self, mouse_id: &str) -> Option<Mouse> {
        let mouse = mice
            .find(mouse_id)
            .get_result::<Mouse>(&mut self.pool.get().unwrap())
            .expect("Error loading mouse by id.");
        Some(mouse)
    }

    pub fn update_mouse_by_id(&self, mouse_id: &str, updated_mouse: Mouse) -> Option<Mouse> {
        let mouse = diesel::update(mice.find(mouse_id))
            .set(&updated_mouse)
            .get_result::<Mouse>(&mut self.pool.get().unwrap())
            .expect("Error updating mouse by id");
        Some(mouse)
    }

    pub fn delete_mouse_by_id(&self, mouse_id: &str) -> Option<usize> {
        let count = diesel::delete(mice.find(mouse_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting mouse by id");
        Some(count)
    }
}