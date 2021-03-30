// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` connection

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::future::FutureExt;
use reqwest::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    Client, ClientBuilder, Url,
};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::{
    model::{DatabaseCurrent, Response},
    Database,
};

async fn to_json<T>(res: reqwest::Response) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.error_for_status()
        .map(|res| async move { res.json::<T>().await })?
        .await
}

async fn handle_response<T>(res: Result<reqwest::Response, Error>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    res.map(to_json)?.await
}

/// An ArangoDB connection
#[derive(Clone, Debug)]
pub struct Connection {
    #[doc(hidden)]
    url: Url,
    #[doc(hidden)]
    client: Client,
}

#[async_trait]
impl Database for Connection {
    async fn current(&self) -> Result<Response<DatabaseCurrent>> {
        let current_url = self
            .url
            .join("_api/database/current")
            .with_context(|| "Unable to build 'current' url")?;
        Ok(self
            .client
            .get(current_url)
            .send()
            .then(handle_response)
            .await?)
    }
}

/// An ArangoDB connection builder
#[derive(Clone, Debug, Default)]
pub struct ConnectionBuilder {
    url: String,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

impl ConnectionBuilder {
    /// Create a new connection builder
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
    pub async fn build(self) -> Result<Connection> {
        let mut headers = HeaderMap::new();
        let _ = headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

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
        let auth_res: AuthResponse<String> = tmp_client
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
        let _ = headers.insert(AUTHORIZATION, HeaderValue::from_bytes(bearer.as_bytes())?);

        // Setup the client
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .with_context(|| "Unable to build the client")?;

        Ok(Connection { url, client })
    }
}

#[derive(Serialize)]
struct AuthBody {
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct AuthResponse<T>
where
    T: Into<String>,
{
    jwt: T,
}

#[cfg(test)]
impl From<&str> for AuthResponse<String> {
    fn from(val: &str) -> AuthResponse<String> {
        Self {
            jwt: val.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AuthResponse, Connection, ConnectionBuilder};
    use crate::{model::Response, traits::Database};
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
        let body: AuthResponse<String> = "not a real jwt".into();
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
        let body = Response::default();
        let mock_response = ResponseTemplate::new(200).set_body_json(body);

        Mock::given(method("GET"))
            .and(path("/_db/keti/_api/database/current"))
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
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
