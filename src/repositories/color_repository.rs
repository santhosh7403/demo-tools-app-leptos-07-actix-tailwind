use crate::models::color::Color;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
    actix_web::web::Data,
    leptos_actix::extract,
    sqlx::{Pool, Sqlite},
    uuid::Uuid,
};

#[server(AppendColor)]
pub async fn append_color(name: String, hexcode: String) -> Result<Color, ServerFnError> {
    use crate::components::shared::common_functions::uppercase_first_letter;
    let id = Uuid::new_v4().to_string();
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let name = uppercase_first_letter(&name);
    let pool = conn.into_inner();

    let _res: Vec<Color> =
        sqlx::query_as("INSERT INTO color (id, name, hex_code) VALUES ($1, $2, $3)")
            .bind(&id)
            .bind(&name)
            .bind(&hexcode.to_uppercase())
            .fetch_all(&*pool)
            .await?;
    Ok(Color { id, name, hexcode })
}

#[server(RemoveColor)]
pub async fn remove_color(id: String) -> Result<(), ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    let _res: Vec<Color> = sqlx::query_as("DELETE FROM color WHERE id = $1")
        .bind(&id)
        .fetch_all(&*pool)
        .await?;
    Ok(())
}

#[server(AllColors)]
pub async fn all_colors() -> Result<Vec<Color>, ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    let res: Vec<Color> = sqlx::query_as("SELECT id, name, hex_code as hexcode FROM color")
        .fetch_all(&*pool)
        .await?;

    // leptos::logging::log!("All colors: {:?}", res);
    Ok(res)
}
