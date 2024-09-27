use std::net::TcpListener;
use sqlx::{Pool, Postgres};


#[sqlx::test(migrations = "./migrations")]
async fn health_check_works(pool: Pool<Postgres>) -> sqlx::Result<()>{

    let app = spawn_app(pool).await;
    let client = reqwest::Client::new();

    let response = client
    .get(format!("{}/health_check", app.address))
    .send()
    .await
    .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

// Test case: form data is fine
#[sqlx::test(migrations = "./migrations")]
async fn subscribe_returns_200_for_valid_form_data(pool: Pool<Postgres>) -> sqlx::Result<()> {

    let app = spawn_app(pool).await;
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

    Ok(())
}

//Test case: form data is missing a portion
#[sqlx::test(migrations = "./migrations")]
async fn subscribe_returns_400_when_data_is_missing(pool: Pool<Postgres>) -> sqlx::Result<()>{
    let app = spawn_app(pool).await;
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
    Ok(())
}

pub struct TestApp {
    pub address: String,
    pub db_pool: Pool<Postgres>
}

async fn spawn_app(pool: Pool<Postgres>) -> TestApp {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to an available port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = zero2prod::startup::run(listener, pool.clone())
        .expect("Failed to bind server to address");
    let _ = tokio::spawn(server);

    TestApp{
        address,
        db_pool: pool
    }
}