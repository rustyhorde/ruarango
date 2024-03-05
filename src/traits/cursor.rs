// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Cursor operations trait

use crate::{
    cursor::output::CursorMeta,
    model::cursor::input::{CreateConfig, DeleteConfig, NextConfig},
    ArangoResult,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Cursor Operations
#[async_trait]
#[allow(unused_qualifications)]
pub trait Cursor {
    /// Create a cursor
    async fn create<T>(&self, config: CreateConfig) -> ArangoResult<CursorMeta<T>>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    /// Delete a cursor
    async fn delete(&self, config: DeleteConfig) -> ArangoResult<()>;

    /// Grab the next batch from an open cursor
    async fn next<T>(&self, config: NextConfig) -> ArangoResult<CursorMeta<T>>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
}
