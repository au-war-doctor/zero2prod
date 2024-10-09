use actix_web::HttpResponse;
use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
//use tracing_futures::Instrument;
use web::{Form, Data};


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    form: &Form<FormData>,
    pool: &Data<PgPool>
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
        )
        .execute(pool.as_ref()) //immutable reference to pgconnection wrapped by web::Data
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute postgres query: {:?}", e);
            e
        })?;
        Ok(())
}


// Serde will attempt to extract to FormData, and if all componensts do not succeed,
// the handler won't even be called and we won't get an OK anyhow
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: Form<FormData>,
    pool: Data<PgPool>) -> HttpResponse {

    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
