use actix_web::{test, App};

use tattico::controllers;

#[actix_rt::test]
async fn test_index_post() {
    let mut app = test::init_service(App::new().configure(controllers::routes)).await;
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_client_error());
}
