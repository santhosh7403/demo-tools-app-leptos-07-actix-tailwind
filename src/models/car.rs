use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Car {
    pub id: String,
    pub make: String,
    pub model: String,
    pub color: String,
    pub year: u16,
    pub price: f32,
}
