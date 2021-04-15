// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document operations trait

use super::Either;
use crate::doc::{
    input::{Config, ReadConfig},
    output::DocMeta,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Document Operations
#[async_trait]
pub trait Document {
    /// Create a document
    async fn create<T, U, V>(
        &self,
        collection: &str,
        config: Config,
        document: &T,
    ) -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Read a document
    async fn read<T>(
        &self,
        collection: &str,
        key: &str,
        config: ReadConfig,
    ) -> Result<Either<libeither::Either<(), T>>>
    where
        T: DeserializeOwned + Send + Sync;

    /// Update the given docment with the given data
    async fn update<T, U, V>() -> Result<Either<DocMeta<U, V>>>;
}
