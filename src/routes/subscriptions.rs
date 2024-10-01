use actix_web::HttpResponse;
use actix_web::web;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing_futures::Instrument;

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

    let request_span = tracing::info_span!("Adding a new subscriber", %request_id, subscriber_email=%form.email, subscriber_name=%form.name);
    let _request_span_guard = request_span.enter();// uhhhh apparently bad practice. !REMOVE-LATER

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // this tracing crate has 'oh and also send a log' feature... although observability best practices
    // shows the two approaches used in different, not identical, contexts.
    tracing::info!("request_id {} - Adding '{}' '{}' as a new subscriber", request_id, form.email, form.name);
    tracing::info!("request_id {} - Saving new subscriber details in database0", request_id);

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
        .instrument(query_span)
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
