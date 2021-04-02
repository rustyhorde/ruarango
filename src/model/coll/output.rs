// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Collection Output Structs

use super::{CollectionKind, Status};
use getset::Getters;
use serde_derive::Deserialize;
#[cfg(test)]
use serde_derive::Serialize;

macro_rules! coll_output {
    ($(#[$sattr:meta])+ pub struct $name:ident {
        $(
            $(#[$attr:meta])+
            $field:ident: $kind:ty => $val:expr,
        )*
    }) => {
        $(#[$sattr])+
        #[derive(Clone, Debug, Deserialize, Getters)]
        #[cfg_attr(test, derive(Serialize))]
        #[getset(get = "pub")]
        pub struct $name {
            /// Is this respone an error?
            error: bool,
            /// The response code, i.e. 200, 404
            code: usize,
            $(
                $(#[$attr])+
                $field: $kind
            ),*
        }

        #[cfg(test)]
        impl Default for $name {
            fn default() -> Self {
                Self {
                    error: false,
                    code: 200,
                    $($field: $val),*
                }
            }
        }
    };
}

coll_output!(
    /// Output when [`load`](crate::Collection::load) is called for a collection
    #[derive(Copy)]
    pub struct Load {
        /// The number of documents inside the collection. This is only
        /// returned if `include_count` is set to true when [`load`](crate::Collection::load) is
        /// called
        count: usize => 10,
    }
);

coll_output!(
    /// Output when [`revision`](crate::Collection::revision) is called for a collection
    pub struct Revision {
        /// The `revision` is a server-generated string that clients can use to check
        /// whether data in a collection has changed since the last
        /// [`revision`](crate::Collection::revision) check.
        revision: String => "1695597447239172096".to_string(),
    }
);

coll_output!(
    /// Output when [`figures`](crate::Collection::figures) is called for a collection
    #[derive(Copy)]
    pub struct Figures {
        /// Figure details
        figures: FiguresDetails => FiguresDetails::default(),
    }
);

coll_output!(
    /// Output when [`count`](crate::Collection::count) is called for a collection
    #[derive(Copy)]
    pub struct Count {
        /// The number of documents inside the collection.
        count: usize => 10,
    }
);

coll_output!(
    /// Output when [`checksum`](crate::Collection::checksum) is called for a collection
    pub struct Checksum {
        /// The `checksum` of documents in a collection
        checksum: String => "0".to_string(),
    }
);

coll_output!(
    /// Output when [`drop`](crate::Collection::drop) is called for a collection
    pub struct Drop {
        /// The `id` of the dropped collection
        id: String => "abc".to_string(),
    }
);

coll_output!(
    /// Output when [`load_indexes`](crate::Collection::load_indexes) is called for a collection
    #[derive(Copy)]
    pub struct LoadIndexes {
        /// `true` if the indices were loaded successfully
        result: bool => true,
    }
);

coll_output!(
    /// Output when [`modify_props`](crate::Collection::modify_props) is called for a collection
    pub struct ModifyProps {
        /// New wait for sync
        #[serde(rename = "waitForSync")]
        wait_for_sync: bool => true,
        /// New journal size
        #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
        journal_size: Option<usize> => Some(12000000),
        /// New schema
        #[serde(skip_serializing_if = "Option::is_none")]
        schema: Option<String> => None,
    }
);

coll_output!(
    /// Output when [`recalculate_count`](crate::Collection::recalculate_count) is called for a collection
    #[derive(Copy)]
    pub struct RecalculateCount {
        /// The result
        result: bool => true,
        /// The new count
        count: usize => 10,
    }
);

coll_output!(
    /// Output when [`rename`](crate::Collection::rename) is called for a collection
    pub struct Rename {
        /// The new name
        name: String => "test_boll".to_string(),
    }
);

coll_output!(
    /// Output when [`truncate`](crate::Collection::truncate) is called for a collection
    #[derive(Copy)]
    pub struct Truncate {}
);

coll_output!(
    /// Output when [`unload`](crate::Collection::unload) is called for a collection
    #[derive(Copy)]
    pub struct Unload {}
);

coll_output!(
    /// Output when [`collection`](crate::Collection::collection) is called for a collection
    pub struct Collection {
        /// The id of the current collection
        id: String => "5847".to_string(),
        /// The name of the current collection
        name: String => "keti".to_string(),
        /// The collection status
        status: Status => Status::Loaded,
        /// The collection kind
        #[serde(rename = "type")]
        kind: CollectionKind => CollectionKind::Document,
        /// Is the current collection a `_system` collection
        #[serde(rename = "isSystem")]
        is_system: bool => false,
        /// The globally unique id
        #[serde(rename = "globallyUniqueId")]
        globally_unique_id: String => "hD4537D142F4C/5847".to_string(),
    }
);

coll_output!(
    /// Output when [`create`](crate::Collection::create) is called for a collection
    pub struct Create {
        /// The collection name
        name: String => "test_coll".to_string(),
        /// Whether the collection is used in a SmartGraph (Enterprise Edition
        /// only). (cluster only)
        #[serde(rename = "isSmart", skip_serializing_if = "Option::is_none")]
        is_smart: Option<bool> => None,
        /// Determines an attribute of the collection that must contain the shard
        /// key value of the referred-to SmartJoin collection (Enterprise Edition
        /// only). (cluster only)
        #[serde(rename = "smartJoinAttribute", skip_serializing_if = "Option::is_none")]
        smart_join_attribute: Option<String> => None,
        /// The number of shards of the collection. (cluster only)
        #[serde(rename = "numberOfShards", skip_serializing_if = "Option::is_none")]
        number_of_shards: Option<usize> => None,
        /// Any of: ["unloaded", "loading", "loaded", "unloading", "deleted",
        /// "unknown"] Only relevant for the MMFiles storage engine
        #[serde(rename = "statusString")]
        status_string: String => "loading".to_string(),
        /// Unique identifier of the collection
        #[serde(rename = "globallyUniqueId")]
        globally_unique_id: String => "abcdef".to_string(),
        /// unique identifier of the collection; deprecated
        id: String => "abc".to_string(),
        /// Attribute that is used in SmartGraphs (Enterprise Edition only).
        /// (cluster only)
        #[serde(
            rename = "smartGraphAttribute",
            skip_serializing_if = "Option::is_none"
        )]
        smart_graph_attribute: Option<String> => None,
        /// Contains how many copies of each shard are kept on different
        /// DB-Servers. It is an integer number in the range of 1-10 or the
        /// string "satellite" for a SatelliteCollection (Enterprise Edition only).
        /// (cluster only)
        #[serde(rename = "replicationFactor", skip_serializing_if = "Option::is_none")]
        replication_factor: Option<String> => None,
        /// Whether or not the collection will be compacted. This option is
        /// only present for the MMFiles storage engine.
        #[serde(rename = "doCompact", skip_serializing_if = "Option::is_none")]
        do_compact: Option<bool> => None,
        /// The collection kind
        #[serde(rename = "type")]
        kind: CollectionKind => CollectionKind::Document,
        /// The number of index buckets. Only relevant for the MMFiles
        /// storage engine
        #[serde(rename = "indexBuckets", skip_serializing_if = "Option::is_none")]
        index_buckets: Option<usize> => None,
        /// The collection level schema for documents.
        #[serde(skip_serializing_if = "Option::is_none")]
        schema: Option<String> => None,
        /// The status
        status: Status => Status::Loaded,
        /// The maximal size setting for journals / datafiles in bytes.
        /// This option is only present for the MMFiles storage engine.
        #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
        journal_size: Option<String> => None,
        /// If true then creating, changing or removing documents
        /// will wait until the data has been synchronized to disk.
        #[serde(rename = "waitForSync")]
        wait_for_sync: bool => false,
        /// true if this is a system collection; usually name will
        /// start with an underscore.
        #[serde(rename = "isSystem")]
        is_system: bool => false,
        /// the sharding strategy selected for the collection.
        /// One of 'hash' or 'enterprise-hash-smart-edge'. (cluster only)
        #[serde(rename = "shardingStrategy", skip_serializing_if = "Option::is_none")]
        sharding_strategy: Option<String> => None,
            /// If true then the collection data will be
        /// kept in memory only and ArangoDB will not write or sync the data
        /// to disk. This option is only present for the MMFiles storage engine.
        #[serde(rename = "isVolatile", skip_serializing_if = "Option::is_none")]
        is_volatile: Option<bool> => None,
        /// Determines how many copies of each shard are required to be
        /// in sync on the different DB-Servers. If there are less then these many copies
        /// in the cluster a shard will refuse to write. Writes to shards with enough
        /// up-to-date copies will succeed at the same time however. The value of
        /// writeConcern can not be larger than replicationFactor. (cluster only)
        #[serde(rename = "writeConcern")]
        write_concern: usize => 0,
        /// Contains the names of document attributes that are used to
        /// determine the target shard for documents. (cluster only)
        #[serde(rename = "shard_keys", skip_serializing_if = "Option::is_none")]
        shard_keys: Option<Vec<String>> => None,
        /// Key Options
        #[serde(rename = "keyOptions")]
        key_options: CreateKeyOptions => CreateKeyOptions::default(),
    }
);

coll_output!(
    /// Output when [`collections`](crate::Collection::collections) is called for a collection
    pub struct Collections {
        /// The id of the current collection
        id: String => "16042".to_string(),
        /// The name of the current collection
        name: String => "edges".to_string(),
        /// The collection status
        status: Status => Status::Loaded,
        /// The collection kind
        #[serde(rename = "type")]
        kind: CollectionKind => CollectionKind::Edges,
        /// Is the current collection a `_system` collection
        #[serde(rename = "isSystem")]
        is_system: bool => false,
        /// The globally unique id
        #[serde(rename = "globallyUniqueId")]
        globally_unique_id: String => "hD4537D142F4C/16042".to_string(),
    }
);

/// Figure details that are part of the [`Figures`](Figures) output
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct FiguresDetails {
    /// Index details
    indexes: FiguresIndexes,
    /// The size of all the documents in bytes
    #[serde(rename = "documentsSize")]
    documents_size: usize,
    /// Is the cache in use?
    #[serde(rename = "cacheInUse")]
    cache_in_use: bool,
    /// Cache size in bytes
    #[serde(rename = "cacheSize")]
    cache_size: usize,
    /// Cache usage in bytes
    #[serde(rename = "cacheUsage")]
    cache_usage: usize,
}

#[cfg(test)]
impl Default for FiguresDetails {
    fn default() -> Self {
        Self {
            indexes: FiguresIndexes::default(),
            documents_size: 0,
            cache_in_use: false,
            cache_size: 0,
            cache_usage: 0,
        }
    }
}

/// Index details that are part of the [`Figures`](Figures) output
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct FiguresIndexes {
    /// The total number of indexes defined for the collection, including the pre-defined
    /// indexes (e.g. primary index).
    count: usize,
    /// The total memory allocated for indexes in bytes
    size: usize,
}

#[cfg(test)]
impl Default for FiguresIndexes {
    fn default() -> Self {
        Self { count: 1, size: 0 }
    }
}

/// Key options that are part of the [`Create`](Create) output
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct CreateKeyOptions {
    /// If set to true, then it is allowed to supply own key values in the
    /// _key attribute of a document. If set to false, then the key generator
    /// will solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    #[serde(rename = "allowUserKeys")]
    allow_user_keys: bool,
    /// The last key value used
    #[serde(rename = "lastValue")]
    last_value: usize,
    /// Specifies the type of the key generator. The currently available generators are
    /// traditional, autoincrement, uuid and padded.
    ///
    /// The traditional key generator generates numerical keys in ascending order.
    /// The autoincrement key generator generates numerical keys in ascending order,
    /// the initial offset and the spacing can be configured (note: autoincrement is currently only
    /// supported for non-sharded collections).
    /// The padded key generator generates keys of a fixed length (16 bytes) in
    /// ascending lexicographical sort order. This is ideal for usage with the RocksDB
    /// engine, which will slightly benefit keys that are inserted in lexicographically
    /// ascending order. The key generator can be used in a single-server or cluster.
    /// The uuid key generator generates universally unique 128 bit keys, which
    /// are stored in hexadecimal human-readable format. This key generator can be used
    /// in a single-server or cluster to generate "seemingly random" keys. The keys
    /// produced by this key generator are not lexicographically sorted.
    #[serde(rename = "type")]
    kind: String,
}

#[cfg(test)]
impl Default for CreateKeyOptions {
    fn default() -> Self {
        Self {
            allow_user_keys: false,
            last_value: 0,
            kind: "traditional".to_string(),
        }
    }
}
