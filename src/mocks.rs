use crate::{common::output::Response, db::output::Current, model::auth::output::AuthResponse};
use wiremock::{
    matchers::{body_string_contains, method, path},
    Mock, MockServer, ResponseTemplate,
};

pub async fn start_mock_server() -> MockServer {
    MockServer::start().await
}

pub async fn mock_auth(mock_server: &MockServer) {
    let body: AuthResponse = "not a real jwt".into();
    let mock_response = ResponseTemplate::new(200).set_body_json(body);

    Mock::given(method("POST"))
        .and(path("/_open/auth"))
        .and(body_string_contains("username"))
        .and(body_string_contains("password"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

pub async fn mock_database_create(mock_server: &MockServer) {
    let body = Response::<Current>::default();
    let mock_response = ResponseTemplate::new(200).set_body_json(body);

    Mock::given(method("GET"))
        .and(path("/_db/test_db/_api/database/current"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

pub async fn mock_async_database_create(mock_server: &MockServer) {
    let mock_response = ResponseTemplate::new(202).insert_header("x-arango-async-id", "123456");

    Mock::given(method("GET"))
        .and(path("/_db/test_db/_api/database/current"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

pub async fn mock_async_ff_database_create(mock_server: &MockServer) {
    let mock_response = ResponseTemplate::new(202);

    Mock::given(method("GET"))
        .and(path("/_db/test_db/_api/database/current"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

pub async fn mock_get_job(mock_server: &MockServer) {
    let mock_response = ResponseTemplate::new(200);

    Mock::given(method("GET"))
        .and(path("/_db/test_db/_api/job/123456"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}

pub async fn mock_put_job(mock_server: &MockServer) {
    let body = Response::<Current>::default();
    let mock_response = ResponseTemplate::new(200).set_body_json(body);

    Mock::given(method("PUT"))
        .and(path("/_db/test_db/_api/job/123456"))
        .respond_with(mock_response)
        .mount(&mock_server)
        .await;
}
