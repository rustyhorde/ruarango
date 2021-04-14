// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Job operations trait

use anyhow::Result;
use async_trait::async_trait;
use libeither::Either;
use serde::{de::DeserializeOwned, Serialize};

/// Collection Operations
#[async_trait]
pub trait Job {
    /// Returns the processing status of the specified job. The processing status
    /// can be determined by checking the HTTP response code.
    ///
    /// * `200` is returned if the job requested via `id` has been executed
    /// and its result is ready to fetch.
    /// * `202` is returned if the job requested via `id` is still in the
    /// queue of pending (or not yet finished) jobs.
    /// * `404` is returned if the job was not found, has already deleted,
    /// has already been fetched from the job result list.
    async fn status(&self, id: &str) -> Result<u16>;

    /// Docs
    async fn fetch<T>(&self, id: &str) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    /// Docs
    async fn fetch_300<T>(&self, id: &str) -> Result<Either<(), T>>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    /// Docs
    async fn jobs(&self, kind: &str) -> Result<Vec<String>>;
}
