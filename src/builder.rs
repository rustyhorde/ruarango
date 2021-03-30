// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` connection builder

use crate::{conn::Connection, utils::handle_response};
use anyhow::{Context, Result};
use futures::future::FutureExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    ClientBuilder, Url,
};
use serde_derive::{Deserialize, Serialize};

/// An `ArangoDB` connection builder
#[derive(Clone, Debug, Default)]
#[allow(clippy::clippy::module_name_repetitions)]
pub struct ConnectionBuilder {
    url: String,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

impl ConnectionBuilder {
    /// Create a new connection builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the url to use for this connection
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: Into<String>,
    {
        self.url = url.into();
        self
    }

    /// Set the username to use for this connection
    pub fn username<T>(mut self, username: T) -> Self
    where
        T: Into<String>,
    {
        self.username = Some(username.into());
        self
    }

    /// Set the password to use for this connection
    pub fn password<T>(mut self, password: T) -> Self
    where
        T: Into<String>,
    {
        self.password = Some(password.into());
        self
    }

    /// Set the database to use for this connection
    pub fn database<T>(mut self, database: T) -> Self
    where
        T: Into<String>,
    {
        self.database = Some(database.into());
        self
    }

    /// Build the connection
    ///
    /// # Errors
    ///
    pub async fn build(self) -> Result<Connection> {
        let mut headers = HeaderMap::new();
        let _old = headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Setup the client to grab a JWT
        let tmp_client = ClientBuilder::new()
            .default_headers(headers.clone())
            .build()
            .with_context(|| "Unable to build the JWT client")?;

        // Generate the auth url
        let base_url = Url::parse(&self.url).with_context(|| "Unable to parse the base url")?;
        let auth_url = base_url
            .join("_open/auth")
            .with_context(|| "Unable to parse the auth url")?;

        // Make the request with the given username/password
        let username = self.username.unwrap_or_else(|| "root".to_string());
        let password = self.password.unwrap_or_default();
        let auth_res: AuthResponse = tmp_client
            .post(auth_url)
            .json(&AuthBody { username, password })
            .send()
            .then(handle_response)
            .await?;

        // Setup the db prefix if necessary
        let url = if let Some(db) = self.database {
            base_url.join(&format!("_db/{}/", db))?
        } else {
            base_url
        };

        // Add the default Authorization header
        let bearer = format!("Bearer {}", auth_res.jwt);
        let _old = headers.insert(AUTHORIZATION, HeaderValue::from_bytes(bearer.as_bytes())?);

        // Setup the client
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .with_context(|| "Unable to build the client")?;

        Ok(Connection::new(url, client))
    }
}

#[derive(Serialize)]
struct AuthBody {
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct AuthResponse {
    jwt: String,
}

#[cfg(test)]
impl From<&str> for AuthResponse {
    fn from(val: &str) -> AuthResponse {
        Self {
            jwt: val.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AuthResponse, Connection, ConnectionBuilder};
    use crate::{db::Current, model::Response, traits::Database};
    use anyhow::Result;
    use wiremock::{
        matchers::{body_string_contains, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn default_conn<T>(uri: T) -> Result<Connection>
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

    async fn mock_auth(mock_server: &MockServer) {
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

    async fn mock_current(mock_server: &MockServer) {
        let body = Response::<Current>::default();
        let mock_response = ResponseTemplate::new(200).set_body_json(body);

        Mock::given(method("GET"))
            .and(path("/_db/keti/_api/database/current"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;
    }

    async fn mock_user(mock_server: &MockServer) {
        let body = Response::<Vec<String>>::default();
        let mock_response = ResponseTemplate::new(200).set_body_json(body);

        Mock::given(method("GET"))
            .and(path("/_db/keti/_api/database/user"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;
    }

    #[tokio::test]
    async fn test_builder() {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        assert!(default_conn(mock_server.uri()).await.is_ok());
    }

    #[tokio::test]
    async fn test_current() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_current(&mock_server).await;

        let conn = default_conn(mock_server.uri()).await?;

        match conn.current().await {
            Ok(res) => {
                assert_eq!(*res.code(), 200);
                assert!(!res.error());
                assert_eq!(res.result().name(), "test");
                assert_eq!(res.result().id(), "123");
                assert!(!res.result().is_system());
                assert_eq!(res.result().path(), "abcdef");
                assert!(res.result().sharding().is_none());
                assert!(res.result().replication_factor().is_none());
                assert!(res.result().write_concern().is_none());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    #[tokio::test]
    async fn test_user() -> Result<()> {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        mock_user(&&mock_server).await;

        let conn = default_conn(mock_server.uri()).await?;

        match conn.user().await {
            Ok(res) => {
                assert_eq!(*res.code(), 200);
                assert!(!res.error());
                assert!(res.result().len() > 0);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
