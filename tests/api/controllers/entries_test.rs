use tattico::controllers;

use crate::post;

#[actix_rt::test]
async fn test_index_post() {
    let resp = post(controllers::routes, "/").await;
    assert!(resp.status().is_client_error());
}
