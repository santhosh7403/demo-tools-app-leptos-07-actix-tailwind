use crate::models::car::Car;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
    actix_web::web::Data,
    leptos_actix::extract,
    sqlx::{Pool, Sqlite},
    uuid::Uuid,
};

#[server(AppendCar)]
pub async fn append_car(
    make: String,
    model: String,
    year: u16,
    color: String,
    price: f32,
) -> Result<Car, ServerFnError> {
    use crate::components::shared::common_functions::uppercase_first_letter;

    let id = Uuid::new_v4().to_string();
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    leptos::logging::log!("Inserting into");

    let _res: Vec<Car> = sqlx::query_as(
        "INSERT INTO car (id, make, model, color, year, price) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&id)
    .bind(uppercase_first_letter(&make))
    .bind(uppercase_first_letter(&model))
    .bind(uppercase_first_letter(&color))
    .bind(&year)
    .bind(&price)
    .fetch_all(&*pool)
    .await
    .map_err(|err| {
        leptos::logging::log!("Error: {:?}", err);
        err
    })?;
    // leptos::logging::log!("Inserting {:?}", car.clone());

    Ok(Car {
        id,
        make,
        model,
        color,
        year,
        price,
    })
}

#[server(RemoveCar)]
pub async fn remove_car(id: String) -> Result<(), ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    let _res: Vec<Car> = sqlx::query_as("DELETE FROM car WHERE id = $1")
        .bind(&id)
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            leptos::logging::log!("Error: {:?}", err);
            err
        })?;
    Ok(())
}

#[server(AllCars)]
pub async fn all_cars() -> Result<Vec<Car>, ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    let res: Vec<Car> = sqlx::query_as("SELECT id, make, model, color, year, price FROM car")
        .fetch_all(&*pool)
        .await
        .map_err(|err| {
            leptos::logging::log!("Error: {:?}", err);
            err
        })?;

    // leptos::logging::log!("All cars: {:?}", res);
    Ok(res)
}

#[server(UpdateCar)]
pub async fn update_car(
    id: String,
    make: String,
    model: String,
    year: u16,
    color: String,
    price: f32,
) -> Result<Car, ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    let _res: Vec<Car> = sqlx::query_as(
        "UPDATE car SET make = $1, model = $2, color = $3, year = $4, price = $5 WHERE id = $6",
    )
    .bind(&make)
    .bind(&model)
    .bind(&color)
    .bind(&year)
    .bind(&price)
    .bind(&id)
    .fetch_all(&*pool)
    .await
    .map_err(|err| {
        leptos::logging::log!("Error: {:?}", err);
        err
    })?;
    Ok(Car {
        id,
        make,
        model,
        color,
        year,
        price,
    })
}
