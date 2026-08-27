use zero2prod::run;

async fn spawn_app() -> std::io::Result<()> {
    let server = run().expect("Failed to bind address");
    tokio::spawn(server);
    Ok(())
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
