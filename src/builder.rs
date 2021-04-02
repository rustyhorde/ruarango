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
    error::RuarangoError,
    model::{auth::input::AuthBuilder, auth::output::AuthResponse},
    utils::handle_response,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use futures::future::FutureExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    ClientBuilder, Url,
};

/// An `ArangoDB` connection builder
#[doc(hidden)]
#[derive(Builder, Clone, Debug, Default)]
#[allow(clippy::clippy::module_name_repetitions)]
#[builder(build_fn(skip), pattern = "immutable")]
pub struct Connection {
    ///
    #[builder(setter(into))]
    url: String,
    ///
    #[builder(setter(into, strip_option), default)]
    username: Option<String>,
    ///
    #[builder(setter(into, strip_option), default)]
    password: Option<String>,
    ///
    #[builder(setter(into, strip_option), default)]
    database: Option<String>,
}

impl ConnectionBuilder {
    /// Build the connection
    ///
    /// # Errors
    ///
    pub async fn build(self) -> Result<Conn> {
        let mut headers = HeaderMap::new();
        let _old = headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Setup the client to grab a JWT
        let tmp_client = ClientBuilder::new()
            .default_headers(headers.clone())
            .build()
            .with_context(|| "Unable to build the JWT client")?;

        // Generate the auth url
        let url = self.url.ok_or(RuarangoError::InvalidConnectionUrl)?;
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
            base_url.clone().join(&format!("_db/{}/", db))?
        } else {
            base_url.clone()
        };

        // Add the default Authorization header
        let bearer = format!("Bearer {}", auth_res.jwt());
        let _old = headers.insert(AUTHORIZATION, HeaderValue::from_bytes(bearer.as_bytes())?);

        // Setup the client
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .with_context(|| "Unable to build the client")?;

        Ok(Conn::new(base_url, db_url, client))
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
