// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Collection operations trait

use crate::{
    coll::{
        input::{Config, Props},
        output::{
            Checksum, Collection as Coll, Collections, Count, Create, Drop, Figures, Load,
            LoadIndexes, ModifyProps, RecalculateCount, Rename, Revision, Truncate, Unload,
        },
    },
    common::output::Response,
    traits::Either,
};
use anyhow::Result;
use async_trait::async_trait;

/// Collection Operations
#[async_trait]
pub trait Collection {
    /// Returns a vector of collection descriptions
    ///
    /// Setting `exclude_system` to true will exclude all system collections
    /// from the output.
    async fn collections(&self, exclude_system: bool)
        -> Result<Either<Response<Vec<Collections>>>>;

    /// Return information about a single collection
    async fn collection(&self, name: &str) -> Result<Coll>;

    /// Create a collection
    async fn create(&self, config: &Config) -> Result<Create>;

    /// Drop a collection
    async fn drop(&self, name: &str, is_system: bool) -> Result<Drop>;

    /// Will calculate a checksum of the meta-data (keys and optionally revision ids and
    /// optionally the document data) in the collection.
    ///
    /// The checksum can be used to compare if two collections on different ArangoDB
    /// instances containing the same contents. The current revision of the collection is
    /// returned too so one can make sure the checksums are calculated for the same
    /// state of data.
    ///
    /// By default, the checksum will only be calculated on the `_key` system attribute
    /// of the documents contained in the collection. For edge collections, the system
    /// attributes `_from` and `_to` will also be included in the calculation.
    ///
    /// Setting `with_revisions` to true will include revision ids
    /// (_rev system attributes) in the checksum.
    ///
    /// Setting `with_data` to true will include the user-defined document attributes
    /// in the checksum
    ///
    /// **Note**: Including user-defined attributes will make the checksumming slower.
    /// **Note**: this method is not available in a cluster.
    async fn checksum(&self, name: &str, with_revisions: bool, with_data: bool)
        -> Result<Checksum>;

    /// The number of documents in the collection.
    /// **Note** - this will always load the collection into memory.
    async fn count(&self, name: &str) -> Result<Count>;

    /// Some figures and additional statistical information about the collection.
    async fn figures(&self, name: &str) -> Result<Figures>;

    /// Get the revision id for a collection
    /// The revision id is a server-generated string that clients can use to
    /// check whether data in a collection has changed since the last revision check.
    async fn revision(&self, name: &str) -> Result<Revision>;

    /// Loads a collection into memory.
    async fn load(&self, name: &str, include_count: bool) -> Result<Load>;

    /// `load_indexes` tries to cache all index entries of this collection into memory.
    ///
    /// This will iterate over all indexes of the collection and store the indexed values,
    /// not the entire document data in memory.
    ///
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
    ///
    /// If the index is larger than your memory limit this function will fill up values
    /// up to this limit and for the time being there is no way to control which indexes
    /// of the collection should have priority over others.
    async fn load_indexes(&self, name: &str) -> Result<LoadIndexes>;

    /// Change the properties of a collection
    ///
    /// **Note**: except for `wait_for_sync`, `journal_size` and `schema`, collection
    /// properties cannot be changed once a collection is created. To rename
    /// a collection, the [`rename`](crate::traits::Collection::rename) endpoint must be used.
    async fn modify_props(&self, name: &str, props: Props) -> Result<ModifyProps>;

    /// Recalculates the document count of a collection, if it ever becomes inconsistent.
    ///
    /// **Note**: this method is specific for the RocksDB storage engine
    async fn recalculate_count(&self, name: &str) -> Result<RecalculateCount>;

    /// Renames a collection
    async fn rename(&self, name: &str, new_name: &str) -> Result<Rename>;

    /// Removes all documents from the collection, but leaves the indexes intact.
    async fn truncate(&self, name: &str) -> Result<Truncate>;

    /// Removes a collection from memory. This call does not delete any documents.
    /// You can use the collection afterwards, in which case it will be loaded into
    /// memory.
    async fn unload(&self, name: &str) -> Result<Unload>;
}
