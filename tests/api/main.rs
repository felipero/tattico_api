use std::fs::File;
use std::io::Read;
use std::net::TcpListener;

use reqwest::Client;

mod controllers;

#[actix_rt::test]
async fn health_check_works() {
    let (client, address) = setup();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn index_works() {
    let (client, address) = setup();

    let response = client
        .get(&format!("{}/", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(145), response.content_length());
    assert!(response.text().await.unwrap().contains("This is tattico!"));
}

fn setup() -> (Client, String) {
    let address = spawn_app();

    let mut buf = Vec::new();
    let _ = File::open("ssl/cert.pem").unwrap().read_to_end(&mut buf);

    let client = Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(&buf).unwrap())
        .build()
        .unwrap();
    (client, address)
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();
    let server = tattico::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("https://{}", address)
}
