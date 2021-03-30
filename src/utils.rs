// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` utils

use reqwest::Error;
use serde::de::DeserializeOwned;
#[cfg(test)]
use {
    crate::{
        builder::ConnectionBuilder,
        conn::Connection,
        error::RuarangoError::{self, TestError},
        model::AuthResponse,
    },
    anyhow::Result,
    wiremock::{
        matchers::{body_string_contains, method, path},
        Mock, MockServer, ResponseTemplate,
    },
};

async fn to_json<T>(res: reqwest::Response) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.error_for_status()
        .map(|res| async move { res.json::<T>().await })?
        .await
}

pub(crate) async fn handle_response<T>(res: Result<reqwest::Response, Error>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.map(to_json)?.await
}

#[cfg(test)]
pub(crate) async fn mock_auth(mock_server: &MockServer) {
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

#[cfg(test)]
pub(crate) async fn default_conn<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::new()
        .url(uri)
        .username("root")
        .password("")
        .database("keti")
        .build()
        .await
}

#[cfg(test)]
pub(crate) async fn no_db_conn<T>(uri: T) -> Result<Connection>
where
    T: Into<String>,
{
    ConnectionBuilder::new()
        .url(uri)
        .username("root")
        .password("")
        .build()
        .await
}

#[cfg(test)]
pub(crate) fn to_test_error(val: String) -> RuarangoError {
    TestError { val }
}
