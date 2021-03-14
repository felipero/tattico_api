use actix_web::body::Body;
use actix_web::dev::ServiceResponse;
use actix_web::test::TestRequest;
use actix_web::web::ServiceConfig;
use actix_web::{test, App};

use tattico::health_check;

mod controllers;

#[actix_rt::test]
async fn health_check_works() {
    let mut response = get(health_check, "/health_check").await;
    assert_eq!(response.status(), 200);
    let body = response.take_body();
    let body = body.as_ref().unwrap();
    assert_eq!(&Body::Empty, body);
}

async fn get(routes_config: fn(&mut ServiceConfig), path: &str) -> ServiceResponse<Body> {
    request(routes_config, path, test::TestRequest::get()).await
}

async fn post(routes_config: fn(&mut ServiceConfig), path: &str) -> ServiceResponse<Body> {
    request(routes_config, path, test::TestRequest::post()).await
}

async fn request(
    routes_config: fn(&mut ServiceConfig),
    path: &str,
    method: TestRequest,
) -> ServiceResponse<Body> {
    let app = test::init_service(App::new().configure(routes_config)).await;
    let req = method.uri(path).to_request();
    test::call_service(&app, req).await
}
