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
        ChecksumResponse, CollectionCreate, CountResponse, CreateCollResponse, DropCollResponse,
        FiguresResponse, GetCollResponse, GetCollsResponse, LoadIndexesResponse, LoadResponse,
        Props, PutPropertiesResponse, RecalculateCountResponse, RenameResponse, RevisionResponse,
        TruncateResponse, UnloadResponse,
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
    /// Will calculate a checksum of the meta-data (keys and optionally revision ids) and
    /// optionally the document data in the collection.
    ///
    /// The checksum can be used to compare if two collections on different ArangoDB
    /// instances contain the same contents. The current revision of the collection is
    /// returned too so one can make sure the checksums are calculated for the same
    /// state of data.
    ///
    /// By default, the checksum will only be calculated on the _key system attribute
    /// of the documents contained in the collection. For edge collections, the system
    /// attributes _from and _to will also be included in the calculation.
    ///
    /// By setting the optional query parameter withRevisions to true, then revision
    /// ids (_rev system attributes) are included in the checksumming.
    ///
    /// By providing the optional query parameter withData with a value of true,
    /// the user-defined document attributes will be included in the calculation too.
    /// **Note**: Including user-defined attributes will make the checksumming slower.
    /// **Note**: this method is not available in a cluster.
    async fn checksum(
        &self,
        name: &str,
        with_revisions: bool,
        with_data: bool,
    ) -> Result<ChecksumResponse>;
    /// The number of documents in the collection.
    /// **Note** - this will always load the collection into memory.
    async fn count(&self, name: &str) -> Result<CountResponse>;
    /// Some figures and additional statistical information about the collection.
    async fn figures(&self, name: &str) -> Result<FiguresResponse>;
    /// The result will also contain the collection's revision id.
    /// The revision id is a server-generated
    /// string that clients can use to check whether data in a collection
    /// has changed since the last revision check.
    async fn revision(&self, name: &str) -> Result<RevisionResponse>;
    /// Loads a collection into memory.
    async fn load(&self, name: &str, count: bool) -> Result<LoadResponse>;
    /// This route tries to cache all index entries
    /// of this collection into the main memory.
    /// Therefore it iterates over all indexes of the collection
    /// and stores the indexed values, not the entire document data,
    /// in memory.
    /// All lookups that could be found in the cache are much faster
    /// than lookups not stored in the cache so you get a nice performance boost.
    /// It is also guaranteed that the cache is consistent with the stored data.
    ///
    /// For the time being this function is only useful on RocksDB storage engine,
    /// as in MMFiles engine all indexes are in memory anyways.
    ///
    /// On RocksDB this function honors all memory limits, if the indexes you want
    /// to load are smaller than your memory limit this function guarantees that most
    /// index values are cached.
    /// If the index is larger than your memory limit this function will fill up values
    /// up to this limit and for the time being there is no way to control which indexes
    /// of the collection should have priority over others.
    async fn load_indexes(&self, name: &str) -> Result<LoadIndexesResponse>;

    /// Changes the properties of a collection.
    /// **Note**: except for `waitForSync`, `journalSize` and `schema`, collection
    /// properties cannot be changed once a collection is created. To rename
    /// a collection, the rename endpoint must be used.
    async fn modify_props(&self, name: &str, props: Props) -> Result<PutPropertiesResponse>;
    /// Recalculates the document count of a collection, if it ever becomes inconsistent.
    /// **Note**: this method is specific for the RocksDB storage engine
    async fn recalculate_count(&self, name: &str) -> Result<RecalculateCountResponse>;
    /// Renames a collection
    async fn rename(&self, name: &str, new_name: &str) -> Result<RenameResponse>;
    /// Removes all documents from the collection, but leaves the indexes intact.
    async fn truncate(&self, name: &str) -> Result<TruncateResponse>;
    /// Removes a collection from memory. This call does not delete any documents.
    /// You can use the collection afterwards; in which case it will be loaded into
    /// memory, again.
    async fn unload(&self, name: &str) -> Result<UnloadResponse>;
}
