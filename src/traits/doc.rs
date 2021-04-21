// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document operations trait

use crate::{
    doc::input::{
        CreateConfig, CreatesConfig, DeleteConfig, ReadConfig, ReadsConfig, ReplaceConfig,
        UpdateConfig, UpdatesConfig,
    },
    types::{ArangoResult, ArangoVecResult, DocMetaResult, DocMetaVecResult},
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Document Operations
#[async_trait]
pub trait Document {
    /// Create a document
    async fn create<T, U, V>(&self, config: CreateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Create multiple documents
    async fn creates<T, U, V>(&self, config: CreatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Read a document
    async fn read<T>(&self, config: ReadConfig) -> ArangoResult<T>
    where
        T: DeserializeOwned + Send + Sync;

    /// Read multiple documents
    async fn reads<T, U>(&self, config: ReadsConfig<T>) -> ArangoVecResult<U>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync;

    /// Replace a docment with the given document
    async fn replace<T, U, V>(&self, config: ReplaceConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Replace multiple documents
    async fn replaces<T, U, V>() -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Update the given data in the given document
    async fn update<T, U, V>(&self, config: UpdateConfig<T>) -> DocMetaResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Update the given data in the given documents
    async fn updates<T, U, V>(&self, config: UpdatesConfig<T>) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Delete the given docment
    async fn delete<U, V>(&self, config: DeleteConfig) -> DocMetaResult<U, V>
    where
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;

    /// Deletes the given docments
    async fn deletes<T, U, V>(
        &self,
        collection: &str,
        config: DeleteConfig,
        documents: &[T],
    ) -> DocMetaVecResult<U, V>
    where
        T: Serialize + Send + Sync,
        U: Serialize + DeserializeOwned + Send + Sync,
        V: Serialize + DeserializeOwned + Send + Sync;
}
