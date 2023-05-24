
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::mice)]
pub struct Mouse {
    pub id: String,
    pub brand: String,
    pub model: Option<String>,
    pub width: Vec<Option<f64>>,
    pub height: Vec<Option<f64>>,
    pub length: f64,
    pub weight: f64,
    pub shape: String,
    pub wireless: bool,
    pub sensor: Option<String>,
    pub mcu: Option<String>,
    pub dpi: Option<String>,
    pub polling_rate: Option<String>,
    pub switches: Option<String>,
    pub mouse_wheel_encoder: Option<String>,
    pub material: String,
    pub launch_date: Option<chrono::NaiveDateTime>,
}