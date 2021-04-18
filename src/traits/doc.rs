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
    input::{Config, DeleteConfig, ReadConfig, ReplaceConfig},
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

    /// Create multiple documents
    async fn creates<T, U, V>(
        &self,
        collection: &str,
        config: Config,
        documents: &[T],
    ) -> Result<Either<Vec<DocMeta<U, V>>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Read a document
    async fn read<T>(&self, collection: &str, key: &str, config: ReadConfig) -> Result<Either<T>>
    where
        T: DeserializeOwned + Send + Sync;

    /// Read multiple documents
    async fn reads<T>() -> Result<Either<Vec<T>>>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    /// Replace a docment with the given document
    async fn replace<T, U, V>(
        &self,
        collection: &str,
        key: &str,
        config: ReplaceConfig,
    ) -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Replace multiple documents
    async fn replaces<T, U, V>() -> Result<Either<Vec<DocMeta<U, V>>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Add/Replace the given data in the given document
    async fn update<T, U, V>() -> Result<Either<DocMeta<U, V>>>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Delete the given docment
    async fn delete<U, V>(
        &self,
        collection: &str,
        key: &str,
        config: DeleteConfig,
    ) -> Result<Either<DocMeta<U, V>>>
    where
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;
}
