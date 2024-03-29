#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8001/health")
        .send()
        .await
        .expect("Failed to execute request.");

    println!("{:?}", response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:8001").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
