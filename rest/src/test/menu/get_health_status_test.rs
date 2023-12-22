use actix_web::http::StatusCode;

use crate::main::menu::get_health_status::get_health_status;

#[actix_web::test]
async fn created_successfully() {
    let resp = get_health_status().await;
    assert_eq!(resp.status(), StatusCode::OK);
}
