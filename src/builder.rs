// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` connection builder

use crate::{
    conn::Connection,
    model::{AuthBody, AuthResponse},
    utils::handle_response,
};
use anyhow::{Context, Result};
use futures::future::FutureExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    ClientBuilder, Url,
};

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
        let db_url = if let Some(db) = self.database {
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

        Ok(Connection::new(base_url, db_url, client))
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
