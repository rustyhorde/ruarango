// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor trait implementation

use super::EMPTY_BODY;
use crate::{
    cursor::{output::CursorMeta, BASE_CURSOR_SUFFIX},
    model::{
        cursor::input::{CreateConfig, DeleteConfig},
        BuildUrl,
    },
    utils::{cursor_resp, empty},
    ArangoResult, Connection, Cursor,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
impl Cursor for Connection {
    async fn create<T>(&self, config: CreateConfig) -> ArangoResult<CursorMeta<T>>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        let url = config.build_url(BASE_CURSOR_SUFFIX, self)?;
        self.post(url, None, config, cursor_resp).await
    }

    async fn delete(&self, config: DeleteConfig) -> ArangoResult<()> {
        let url = config.build_url(BASE_CURSOR_SUFFIX, self)?;
        self.delete(url, None, EMPTY_BODY, empty).await
    }
}
