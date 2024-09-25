use std::net::TcpListener;
use sqlx::PgPool;

use zero2prod::configuration::get_configuration;

#[actix_rt::test]
async fn health_check_works(){

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
    .get(format!("{}/health_check", app.address))
    .send()
    .await
    .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

// Test case: form data is fine
#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {

    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // response might come back as a string, careful with types there
    assert_eq!(200, response.status().as_u16());
}

// Test case: form data is missing a portion
#[actix_rt::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")];

    for (body, error) in test_cases {
        let response = client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(400, response.status().as_u16(),
            "The API should have failed with 400 Bad Request but didn't: {}", error);
    }


}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to an available port");

    let port = listener.local_addr().unwrap().port();
    let config = get_configuration().expect("Failed to read from config file");
    let address = format!("http://127.0.0.1:{}", port);

    let pool = PgPool::connect(&config.database.connection_string())
                   .await
                   .expect("Failed to connect to postgres");

    let server = zero2prod::startup::run(listener, pool.clone())
        .expect("Failed to bind server to address");
    let _ = tokio::spawn(server);

    TestApp{
        address,
        db_pool: pool
    }
}