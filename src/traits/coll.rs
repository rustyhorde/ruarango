// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` collection trait

use crate::model::{
    coll::{
        CollectionCreate, CreateCollResponse, DropCollResponse, GetCollResponse, GetCollsResponse,
    },
    Response,
};
use anyhow::Result;
use async_trait::async_trait;

/// Collection related operations
#[async_trait]
pub trait Collection {
    /// Returns an object with an attribute collections containing an
    /// array of all collection descriptions. The same information is also
    /// available in the names as an object with the collection names
    /// as keys.
    ///
    /// By providing the optional query parameter `excludeSystem` with a value of
    /// true, all system collections will be excluded from the response.
    ///
    /// **Warning**:
    /// Accessing collections by their numeric ID is deprecated from version 3.4.0 on.
    /// You should reference them via their names instead.
    async fn collections(&self, exclude_system: bool) -> Result<Response<Vec<GetCollsResponse>>>;
    /// Return information about a collection
    async fn collection(&self, name: &str) -> Result<GetCollResponse>;
    /// Create a collection
    async fn create(&self, collection: &CollectionCreate) -> Result<CreateCollResponse>;
    /// Drop a collection
    async fn drop(&self, name: &str, is_system: bool) -> Result<DropCollResponse>;
}
