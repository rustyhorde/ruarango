// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Job trait implementation

use crate::{
    api_get, api_put,
    conn::Connection,
    traits::Job,
    utils::{doc_resp, handle_response},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use const_format::concatcp;
use futures::FutureExt;
use serde::{de::DeserializeOwned, Serialize};

const BASE_SUFFIX: &str = "_api/job";
const DONE_SUFFIX: &str = concatcp!(BASE_SUFFIX, "/done#by-type");

#[async_trait]
impl Job for Connection {
    async fn status(&self, id: &str) -> Result<u16> {
        let job_id_url = format!("{BASE_SUFFIX}/{id}");
        let current_url = self
            .db_url()
            .join(&job_id_url)
            .with_context(|| format!("Unable to build '{job_id_url}' url"))?;
        let res = self.client().get(current_url).send().await?;
        Ok(res.status().as_u16())
    }

    async fn fetch<T>(&self, id: &str) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        api_put!(self, db_url, &format!("{BASE_SUFFIX}/{id}"))
    }

    async fn fetch_doc_job<T>(&self, id: &str) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        api_put!(self, db_url, &format!("{BASE_SUFFIX}/{id}") => doc_resp)
    }

    async fn jobs(&self, _kind: &str) -> Result<Vec<String>> {
        api_get!(self, db_url, DONE_SUFFIX)
    }
}
