// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` connection builder

use crate::{
    conn::Connection as Conn,
    error::RuarangoErr::InvalidConnectionUrl,
    model::{auth::input::AuthBuilder, auth::output::AuthResponse},
    utils::handle_response,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use futures::future::FutureExt;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION},
    ClientBuilder, Url,
};

/// The kind of asynchronouse request you would like to make
#[derive(Clone, Copy, Debug)]
pub enum AsyncKind {
    /// This will add the HTTP header `x-arango-async: true` to client requests
    /// and `ArangoDB` will put the request into an in-memory task queue and return an
    /// HTTP 202 (accepted) response to the client instantly.
    FireAndForget,
    /// This will add the HTTP header `x-arango-async: store` to a request.
    /// Clients can instruct the `ArangoDB` server to execute the operation
    /// asynchronously as with [`FireAndForget`](Self::FireAndForget), but also
    /// store the operation result in memory for a later retrieval.
    Store,
}

impl Default for AsyncKind {
    fn default() -> Self {
        Self::Store
    }
}

/// An `ArangoDB` connection builder
#[doc(hidden)]
#[derive(Builder, Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
#[builder(build_fn(skip), pattern = "immutable")]
#[allow(dead_code)]
pub struct Connection {
    /// The url used to connect to `ArangoDB`
    #[builder(setter(into))]
    url: String,
    /// An optional username, defaults to 'root'
    #[builder(setter(into, strip_option), default)]
    username: Option<String>,
    /// An optional password, defaults to ''
    #[builder(setter(into, strip_option), default)]
    password: Option<String>,
    /// An optional database to use, defaults to '' which will target the '_system' database
    #[builder(setter(into, strip_option), default)]
    database: Option<String>,
    /// Make this request asynchronously
    #[builder(setter(strip_option), default)]
    async_kind: Option<AsyncKind>,
}

impl ConnectionBuilder {
    /// Build the connection
    ///
    /// # Errors
    /// An invalid url will cause the build to error.
    pub async fn build(self) -> Result<Conn> {
        let mut headers = HeaderMap::new();
        let _old = headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Setup the client to grab a JWT
        let tmp_client = ClientBuilder::new()
            .default_headers(headers.clone())
            .build()
            .with_context(|| "Unable to build the JWT client")?;

        // Generate the auth url
        let url = self.url.ok_or(InvalidConnectionUrl)?;
        let base_url = Url::parse(&url).with_context(|| "Unable to parse the base url")?;
        let auth_url = base_url
            .join("_open/auth")
            .with_context(|| "Unable to parse the auth url")?;

        // Make the request with the given username/password
        let username = self
            .username
            .unwrap_or_else(|| Some("root".to_string()))
            .unwrap_or_default();
        let password = self.password.unwrap_or_default().unwrap_or_default();
        let auth_res: AuthResponse = tmp_client
            .post(auth_url)
            .json(
                &AuthBuilder::default()
                    .username(username)
                    .password(password)
                    .build()?,
            )
            .send()
            .then(handle_response)
            .await?;

        // Setup the db prefix if necessary
        let db_url = if let Some(Some(db)) = self.database {
            base_url.clone().join(&format!("_db/{db}/"))?
        } else {
            base_url.clone()
        };

        // Add any default headers
        let bearer = format!("bearer {}", auth_res.jwt());
        let _old = headers.insert(AUTHORIZATION, HeaderValue::from_bytes(bearer.as_bytes())?);

        let mut is_async = false;
        let mut async_headers = headers.clone();
        if let Some(Some(async_kind)) = self.async_kind {
            is_async = true;
            match async_kind {
                AsyncKind::FireAndForget => {
                    let _old = async_headers.insert(
                        HeaderName::from_static("x-arango-async"),
                        HeaderValue::from_static("true"),
                    );
                }
                AsyncKind::Store => {
                    let _old = async_headers.insert(
                        HeaderName::from_static("x-arango-async"),
                        HeaderValue::from_static("store"),
                    );
                }
            }
        }

        // Setup the client
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .with_context(|| "Unable to build the client")?;

        let async_client = ClientBuilder::new()
            .default_headers(async_headers)
            .build()
            .with_context(|| "Unable to build the async_client")?;

        Ok(Conn::new(base_url, db_url, client, async_client, is_async))
    }
}

#[cfg(test)]
mod test {
    use crate::utils::{default_conn, mock_auth};
    use wiremock::MockServer;

    #[tokio::test]
    async fn test_builder() {
        let mock_server = MockServer::start().await;
        mock_auth(&mock_server).await;
        assert!(default_conn(mock_server.uri()).await.is_ok());
    }
}
