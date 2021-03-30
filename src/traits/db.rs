// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database impl

use super::Database;
use crate::{
    conn::Connection,
    model::{db::Current, Response},
    utils::handle_response,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::future::FutureExt;

#[async_trait]
impl Database for Connection {
    async fn current(&self) -> Result<Response<Current>> {
        let current_url = self
            .url()
            .join("_api/database/current")
            .with_context(|| "Unable to build '_api/database/curren' url")?;
        Ok(self
            .client()
            .get(current_url)
            .send()
            .then(handle_response)
            .await?)
    }

    async fn user(&self) -> Result<Response<Vec<String>>> {
        let current_url = self
            .url()
            .join("_api/database/user")
            .with_context(|| "Unable to build '_api/database/user' url")?;
        Ok(self
            .client()
            .get(current_url)
            .send()
            .then(handle_response)
            .await?)
    }
}
