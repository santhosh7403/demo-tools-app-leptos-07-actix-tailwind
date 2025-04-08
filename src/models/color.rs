use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    pub id: String,
    pub name: String,
    pub hexcode: String,
}
