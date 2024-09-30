use actix_web::HttpResponse;
use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

// Serde will attempt to extract to FormData, and if all componensts do not succeed,
// the handler won't even be called and we won't get an OK anyhow
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>) -> HttpResponse {

    //unique ID to help with post mortem
    let request_id = Uuid::new_v4();

    tracing::info!("request_id {} - Adding '{}' '{}' as a new subscriber", request_id, form.email, form.name);
    tracing::info!("request_id {} - Saving new subscriber details in database0", request_id);

    // Irrespective of success, get logging in there for post mortem purposes
    //log::info!("request_id {} - Adding '{}' '{}' as a new subcriber", request_id, form.email, form.name);

    match sqlx::query!(
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
        {
            Ok(_) => {
                tracing::info!("request_id {} - New subscriber details have been saved", request_id);
                HttpResponse::Ok().finish()
            },
            Err(e) => {
                tracing::error!("request_id {} - Failed to execute postgres query: {:?}", request_id, e);
                HttpResponse::InternalServerError().finish()
            }
        }
}
