// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` collection response models

use derive_builder::Builder;
use getset::{Getters, Setters};
use serde_derive::{Deserialize, Serialize};

/// Collection response data
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct GetCollsResponse {
    /// The id of the current collection
    id: String,
    /// The name of the current collection
    name: String,
    /// The collection status
    ///
    /// 1: new born collection
    /// 2: unloaded
    /// 3: loaded
    /// 4: in the process of being unloaded
    /// 5: deleted
    /// 6: loading
    ///
    /// Every other status indicates a corrupted collection.
    status: usize,
    /// The collection type
    ///
    /// 2: document collection (normal case)
    /// 3: edges collection
    #[serde(rename = "type")]
    kind: usize,
    /// Is the current collection a `_system` collection
    #[serde(rename = "isSystem")]
    is_system: bool,
    /// The globally unique id
    #[serde(rename = "globallyUniqueId")]
    globally_unique_id: String,
}

#[cfg(test)]
impl Default for GetCollsResponse {
    fn default() -> Self {
        Self {
            id: "16042".to_string(),
            name: "edges".to_string(),
            status: 3,
            kind: 3,
            is_system: false,
            globally_unique_id: "hD4537D142F4C/16042".to_string(),
        }
    }
}

/// Collection response data
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct GetCollResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The id of the current collection
    id: String,
    /// The name of the current collection
    name: String,
    /// The collection status
    status: usize,
    /// The collection type
    #[serde(rename = "type")]
    kind: usize,
    /// Is the current collection a `_system` collection
    #[serde(rename = "isSystem")]
    is_system: bool,
    /// The globally unique id
    #[serde(rename = "globallyUniqueId")]
    globally_unique_id: String,
}

#[cfg(test)]
impl Default for GetCollResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            kind: 2,
            status: 3,
            name: "keti".to_string(),
            is_system: false,
            id: "5847".to_string(),
            globally_unique_id: "hD4537D142F4C/5847".to_string(),
        }
    }
}

/// A collection to create
#[derive(Builder, Clone, Debug, Deserialize, Getters, Setters, Serialize)]
#[getset(get = "pub")]
pub struct CollectionCreate {
    /// The collection name
    #[builder(setter(into))]
    name: String,
    /// The maximal size of a journal or datafile in bytes. The value
    /// must be at least 1048576 (1 MiB). (The default is a configuration parameter)
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    journal_size: Option<usize>,
    /// (The default is 1): in a cluster, this attribute determines how many copies
    /// of each shard are kept on different DB-Servers. The value 1 means that only one
    /// copy (no synchronous replication) is kept. A value of k means that k-1 replicas
    /// are kept. It can also be the string "satellite" for a SatelliteCollection,
    /// where the replication factor is matched to the number of DB-Servers
    /// (Enterprise Edition only).
    /// Any two copies reside on different DB-Servers. Replication between them is
    /// synchronous, that is, every write operation to the "leader" copy will be replicated
    /// to all "follower" replicas, before the write operation is reported successful.
    /// If a server fails, this is detected automatically and one of the servers holding
    /// copies take over, usually without an error being reported.
    #[serde(rename = "replicationFactor", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    replication_factor: Option<String>,
    /// Key Options
    #[serde(rename = "keyOptions", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    key_options: Option<KeyOptions>,
    /// If true then the data is synchronized to disk before returning from a
    /// document create, update, replace or removal operation. (default: false)
    #[serde(rename = "waitForSync", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Whether or not the collection will be compacted (default is true)
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(rename = "doCompact", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    do_compact: Option<bool>,
    /// This attribute specifies the name of the sharding strategy to use for
    /// the collection. Since ArangoDB 3.4 there are different sharding strategies
    /// to select from when creating a new collection. The selected shardingStrategy
    /// value will remain fixed for the collection and cannot be changed afterwards.
    /// This is important to make the collection keep its sharding settings and
    /// always find documents already distributed to shards using the same
    /// initial sharding algorithm.
    /// The available sharding strategies are:
    /// community-compat: default sharding used by ArangoDB
    /// Community Edition before version 3.4
    /// enterprise-compat: default sharding used by ArangoDB
    /// Enterprise Edition before version 3.4
    /// enterprise-smart-edge-compat: default sharding used by smart edge
    /// collections in ArangoDB Enterprise Edition before version 3.4
    /// hash: default sharding used for new collections starting from version 3.4
    /// (excluding smart edge collections)
    /// enterprise-hash-smart-edge: default sharding used for new
    /// smart edge collections starting from version 3.4
    /// If no sharding strategy is specified, the default will be hash for
    /// all collections, and enterprise-hash-smart-edge for all smart edge
    /// collections (requires the Enterprise Edition of ArangoDB).
    /// Manually overriding the sharding strategy does not yet provide a
    /// benefit, but it may later in case other sharding strategies are added.
    #[serde(rename = "shardingStrategy", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    sharding_strategy: Option<String>,
    /// If true then the collection data is kept in-memory only and not made persistent.
    /// Unloading the collection will cause the collection data to be discarded. Stopping
    /// or re-starting the server will also cause full loss of data in the
    /// collection. Setting this option will make the resulting collection be
    /// slightly faster than regular collections because ArangoDB does not
    /// enforce any synchronization to disk and does not calculate any CRC
    /// checksums for datafiles (as there are no datafiles). This option
    /// should therefore be used for cache-type collections only, and not
    /// for data that cannot be re-created otherwise.
    /// (The default is false)
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(rename = "isVolatile", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    is_volatile: Option<bool>,
    /// Write concern for this collection (default: 1).
    /// It determines how many copies of each shard are required to be
    /// in sync on the different DB-Servers. If there are less then these many copies
    /// in the cluster a shard will refuse to write. Writes to shards with enough
    /// up-to-date copies will succeed at the same time however. The value of
    /// writeConcern can not be larger than replicationFactor. (cluster only)
    #[serde(rename = "writeConcern", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    write_concern: Option<usize>,
    /// In an Enterprise Edition cluster, this attribute determines an attribute
    /// of the collection that must contain the shard key value of the referred-to
    /// SmartJoin collection. Additionally, the shard key for a document in this
    /// collection must contain the value of this attribute, followed by a colon,
    /// followed by the actual primary key of the document.
    /// This feature can only be used in the Enterprise Edition and requires the
    /// distributeShardsLike attribute of the collection to be set to the name
    /// of another collection. It also requires the shardKeys attribute of the
    /// collection to be set to a single shard key attribute, with an additional ':'
    /// at the end.
    /// A further restriction is that whenever documents are stored or updated in the
    /// collection, the value stored in the smartJoinAttribute must be a string.
    #[serde(rename = "smartJoinAttribute", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    smart_join_attribute: Option<String>,
    ///  (The default is 1): in a cluster, this value determines the
    /// number of shards to create for the collection. In a single
    /// server setup, this option is meaningless.
    #[serde(rename = "numberOfShards", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    number_of_shards: Option<usize>,
    /// (The default is ""): in an Enterprise Edition cluster, this attribute binds
    /// the specifics of sharding for the newly created collection to follow that of a
    /// specified existing collection.
    /// **Note**: Using this parameter has consequences for the prototype
    /// collection. It can no longer be dropped, before the sharding-imitating
    /// collections are dropped. Equally, backups and restores of imitating
    /// collections alone will generate warnings (which can be overridden)
    /// about missing sharding prototype.
    #[serde(
        rename = "distributeShardsLike",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(setter(into, strip_option), default)]
    distribute_shards_like: Option<String>,
    /// If true, create a system collection. In this case collection-name
    /// should start with an underscore. End users should normally create non-system
    /// collections only. API implementors may be required to create system
    /// collections in very special occasions, but normally a regular collection will do.
    /// (The default is false)
    #[serde(rename = "isSystem", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    is_system: Option<bool>,
    /// (The default is 2): the type of the collection to create.
    /// The following values for type are valid:
    /// 2: document collection
    /// 3: edge collection
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    kind: Option<usize>,
    /// (The default is [ "_key" ]): in a cluster, this attribute determines
    /// which document attributes are used to determine the target shard for documents.
    /// Documents are sent to shards based on the values of their shard key attributes.
    /// The values of all shard key attributes in a document are hashed,
    /// and the hash value is used to determine the target shard.
    /// Note: Values of shard key attributes cannot be changed once set.
    /// This option is meaningless in a single server setup.
    #[serde(rename = "shardKeys", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    shard_keys: Option<Vec<String>>,
    /// Optional object that specifies the collection level schema for
    /// documents. The attribute keys rule, level and message must follow the
    /// rules documented in Document Schema Validation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    schema: Option<String>,
}

/// key options for collection response
#[derive(Builder, Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct KeyOptions {
    /// If set to true, then it is allowed to supply own key values in the
    /// _key attribute of a document. If set to false, then the key generator
    /// will solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    #[serde(rename = "allowUserKeys")]
    allow_user_keys: bool,
    /// specifies the type of the key generator. The currently available generators are
    /// traditional, autoincrement, uuid and padded.
    ///
    /// The traditional key generator generates numerical keys in ascending order.
    /// The autoincrement key generator generates numerical keys in ascending order,
    /// the initial offset and the spacing can be configured (note: autoincrement is currently only
    /// +supported for non-sharded collections).
    /// The padded key generator generates keys of a fixed length (16 bytes) in
    /// ascending lexicographical sort order. This is ideal for usage with the RocksDB
    /// engine, which will slightly benefit keys that are inserted in lexicographically
    /// ascending order. The key generator can be used in a single-server or cluster.
    /// The uuid key generator generates universally unique 128 bit keys, which
    /// are stored in hexadecimal human-readable format. This key generator can be used
    /// in a single-server or cluster to generate "seemingly random" keys. The keys
    /// produced by this key generator are not lexicographically sorted.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    kind: String,
    /// increment value for autoincrement key generator. Not used for other key
    /// generator types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    increment: Option<usize>,
    /// Initial offset value for autoincrement key generator.
    /// Not used for other key generator types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    offset: Option<usize>,
}

/// create collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct CreateCollResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The collection name
    name: String,
    /// Whether the collection is used in a SmartGraph (Enterprise Edition
    /// only). (cluster only)
    #[serde(rename = "isSmart", skip_serializing_if = "Option::is_none")]
    is_smart: Option<bool>,
    /// Determines an attribute of the collection that must contain the shard
    /// key value of the referred-to SmartJoin collection (Enterprise Edition
    /// only). (cluster only)
    #[serde(rename = "smartJoinAttribute", skip_serializing_if = "Option::is_none")]
    smart_join_attribute: Option<String>,
    /// The number of shards of the collection. (cluster only)
    #[serde(rename = "numberOfShards", skip_serializing_if = "Option::is_none")]
    number_of_shards: Option<usize>,
    /// Any of: ["unloaded", "loading", "loaded", "unloading", "deleted",
    /// "unknown"] Only relevant for the MMFiles storage engine
    #[serde(rename = "statusString")]
    status_string: String,
    /// Unique identifier of the collection
    #[serde(rename = "globallyUniqueId")]
    globally_unique_id: String,
    /// unique identifier of the collection; deprecated
    id: String,
    /// Attribute that is used in SmartGraphs (Enterprise Edition only).
    /// (cluster only)
    #[serde(
        rename = "smartGraphAttribute",
        skip_serializing_if = "Option::is_none"
    )]
    smart_graph_attribute: Option<String>,
    /// Contains how many copies of each shard are kept on different
    /// DB-Servers. It is an integer number in the range of 1-10 or the
    /// string "satellite" for a SatelliteCollection (Enterprise Edition only).
    /// (cluster only)
    #[serde(rename = "replicationFactor", skip_serializing_if = "Option::is_none")]
    replication_factor: Option<String>,
    /// Whether or not the collection will be compacted. This option is
    /// only present for the MMFiles storage engine.
    #[serde(rename = "doCompact", skip_serializing_if = "Option::is_none")]
    do_compact: Option<bool>,
    /// The kind of the collection:
    ///
    /// 0: "unknown"
    /// 2: regular document collection
    /// 3: edge collection
    #[serde(rename = "type")]
    kind: usize,
    /// The number of index buckets. Only relevant for the MMFiles
    /// storage engine
    #[serde(rename = "indexBuckets", skip_serializing_if = "Option::is_none")]
    index_buckets: Option<usize>,
    /// The collection level schema for documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<String>,
    /// Corresponds to statusString; Only relevant for the MMFiles storage
    /// engine
    ///
    /// 0: "unknown" - may be corrupted
    /// 1: (deprecated, maps to "unknown")
    /// 2: "unloaded"
    /// 3: "loaded"
    /// 4: "unloading"
    /// 5: "deleted"
    /// 6: "loading"
    status: usize,
    /// The maximal size setting for journals / datafiles in bytes.
    /// This option is only present for the MMFiles storage engine.
    #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
    journal_size: Option<String>,
    /// If true then creating, changing or removing documents
    /// will wait until the data has been synchronized to disk.
    #[serde(rename = "waitForSync")]
    wait_for_sync: bool,
    /// true if this is a system collection; usually name will
    /// start with an underscore.
    #[serde(rename = "isSystem")]
    is_system: bool,
    /// the sharding strategy selected for the collection.
    /// One of 'hash' or 'enterprise-hash-smart-edge'. (cluster only)
    #[serde(rename = "shardingStrategy", skip_serializing_if = "Option::is_none")]
    sharding_strategy: Option<String>,
    /// If true then the collection data will be
    /// kept in memory only and ArangoDB will not write or sync the data
    /// to disk. This option is only present for the MMFiles storage engine.
    #[serde(rename = "isVolatile", skip_serializing_if = "Option::is_none")]
    is_volatile: Option<bool>,
    /// Determines how many copies of each shard are required to be
    /// in sync on the different DB-Servers. If there are less then these many copies
    /// in the cluster a shard will refuse to write. Writes to shards with enough
    /// up-to-date copies will succeed at the same time however. The value of
    /// writeConcern can not be larger than replicationFactor. (cluster only)
    #[serde(rename = "writeConcern")]
    write_concern: usize,
    /// Contains the names of document attributes that are used to
    /// determine the target shard for documents. (cluster only)
    #[serde(rename = "shard_keys", skip_serializing_if = "Option::is_none")]
    shard_keys: Option<Vec<String>>,
    /// Key Options
    #[serde(rename = "keyOptions")]
    key_options: KeyOptionsResponse,
}

#[cfg(test)]
impl Default for CreateCollResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            name: "test_coll".to_string(),
            is_smart: None,
            smart_join_attribute: None,
            number_of_shards: None,
            status_string: "loading".to_string(),
            globally_unique_id: "abcdef".to_string(),
            id: "abc".to_string(),
            smart_graph_attribute: None,
            replication_factor: None,
            do_compact: None,
            kind: 1,
            index_buckets: None,
            schema: None,
            status: 6,
            journal_size: None,
            wait_for_sync: false,
            is_system: false,
            sharding_strategy: None,
            is_volatile: None,
            write_concern: 0,
            shard_keys: None,
            key_options: KeyOptionsResponse::default(),
        }
    }
}

/// key options for collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct KeyOptionsResponse {
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
impl Default for KeyOptionsResponse {
    fn default() -> Self {
        Self {
            allow_user_keys: false,
            last_value: 0,
            kind: "traditional".to_string(),
        }
    }
}

/// drop collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct DropCollResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The id of the dropped collection
    id: String,
}

#[cfg(test)]
impl Default for DropCollResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            id: "abc".to_string(),
        }
    }
}

/// checksum collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct ChecksumResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The id of the dropped collection
    id: String,
    /// true if this is a system collection; usually name will
    /// start with an underscore.
    #[serde(rename = "isSystem")]
    is_system: bool,
    /// The kind of the collection:
    ///
    /// 0: "unknown"
    /// 2: regular document collection
    /// 3: edge collection
    #[serde(rename = "type")]
    kind: usize,
    /// The revision
    revision: String,
    /// The checksum
    checksum: String,
    /// The globally unique id
    #[serde(rename = "globallyUniqueId")]
    globally_unique_id: String,
    /// The collection name
    name: String,
    /// The status
    status: usize,
}

#[cfg(test)]
impl Default for ChecksumResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            id: "5847".to_string(),
            is_system: false,
            kind: 2,
            revision: "_cF8MSCu---".to_string(),
            checksum: "0".to_string(),
            globally_unique_id: "hD4537D142F4C/5847".to_string(),
            name: "test_coll".to_string(),
            status: 3,
        }
    }
}

/// count collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct CountResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    // /// The collection name
    // name: String,
    // /// Whether the collection is used in a SmartGraph (Enterprise Edition
    // /// only). (cluster only)
    // #[serde(rename = "isSmartChild")]
    // is_smart_child: bool,
    // /// Is this collection cache enabled
    // #[serde(rename = "cacheEnabled")]
    // is_cache_enabled: bool,
    // /// Any of: ["unloaded", "loading", "loaded", "unloading", "deleted",
    // /// "unknown"] Only relevant for the MMFiles storage engine
    // #[serde(rename = "statusString")]
    // status_string: String,
    // /// Unique identifier of the collection
    // #[serde(rename = "globallyUniqueId")]
    // globally_unique_id: String,
    // /// unique identifier of the collection; deprecated
    // id: String,
    // /// unique identifier of the collection; deprecated
    // #[serde(rename = "objectId")]
    // object_id: String,
    // /// unique identifier of the collection; deprecated
    // #[serde(rename = "tempObjectId")]
    // temp_object_id: String,
    // /// The kind of the collection:
    // ///
    // /// 0: "unknown"
    // /// 2: regular document collection
    // /// 3: edge collection
    // #[serde(rename = "type")]
    // kind: usize,
    // /// The collection level schema for documents.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // schema: Option<String>,
    // /// Corresponds to statusString; Only relevant for the MMFiles storage
    // /// engine
    // ///
    // /// 0: "unknown" - may be corrupted
    // /// 1: (deprecated, maps to "unknown")
    // /// 2: "unloaded"
    // /// 3: "loaded"
    // /// 4: "unloading"
    // /// 5: "deleted"
    // /// 6: "loading"
    // status: usize,
    // /// true if this is a system collection; usually name will
    // /// start with an underscore.
    // #[serde(rename = "isSystem")]
    // is_system: bool,
    // /// If true then the collection is disjoint
    // #[serde(rename = "isDisjoint")]
    // is_disjoint: bool,
    // /// Key Options
    // #[serde(rename = "keyOptions")]
    // key_options: KeyOptionsResponse,
    /// Count
    count: usize,
}

#[cfg(test)]
impl Default for CountResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            count: 10,
        }
    }
}

/// figures collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct FiguresResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// Figures details
    figures: Figures,
}

#[cfg(test)]
impl Default for FiguresResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            figures: Figures::default(),
        }
    }
}

/// Collection figures
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct Figures {
    /// Index details
    indexes: Indexes,
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
impl Default for Figures {
    fn default() -> Self {
        Self {
            indexes: Indexes::default(),
            documents_size: 0,
            cache_in_use: false,
            cache_size: 0,
            cache_usage: 0,
        }
    }
}

/// Index details
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct Indexes {
    /// The total number of indexes defined for the collection, including the pre-defined
    /// indexes (e.g. primary index).
    count: usize,
    /// The total memory allocated for indexes in bytes
    size: usize,
}

#[cfg(test)]
impl Default for Indexes {
    fn default() -> Self {
        Self { count: 1, size: 0 }
    }
}

/// revision collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct RevisionResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The revision id is a server-generated
    /// string that clients can use to check whether data in a collection
    /// has changed since the last revision check.
    revision: String,
}

#[cfg(test)]
impl Default for RevisionResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            revision: "1695597447239172096".to_string(),
        }
    }
}

/// load collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct LoadResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The number of documents inside the collection. This is only
    /// returned if the count input parameters is set to true or has
    /// not been specified.
    count: usize,
}

#[cfg(test)]
impl Default for LoadResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            count: 10,
        }
    }
}

/// Should a count happen on puts
#[derive(Builder, Clone, Copy, Debug, Setters, Serialize)]
pub struct ShouldCount {
    /// If set, this controls whether the return value should include
    /// the number of documents in the collection. Setting count to
    /// false may speed up loading a collection. The default value for
    /// count is true.
    count: bool,
}

/// load collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct LoadIndexesResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// true on success
    result: bool,
}

#[cfg(test)]
impl Default for LoadIndexesResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            result: true,
        }
    }
}

/// Collection properties to modify
#[derive(Builder, Clone, Debug, Setters, Serialize)]
pub struct Props {
    /// If true then creating or changing a document
    /// will wait until the data has been synchronized to disk.
    #[serde(rename = "waitForSync", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// The maximal size of a journal or datafile in bytes.
    /// The value must be at least 1048576 (1 MB). Note that when
    /// changing the `journalSize` value, it will only have an effect for
    /// additional journals or datafiles that are created. Already
    /// existing journals or datafiles will not be affected.
    #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    journal_size: Option<usize>,
    /// Object that specifies the collection level schema
    /// for documents. The attribute keys rule, level and message must follow
    /// the rules documented in Document Schema Validation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    schema: Option<String>,
}

/// load collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct PutPropertiesResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// New wait for sync
    #[serde(rename = "waitForSync")]
    wait_for_sync: bool,
    /// New journal size
    #[serde(rename = "journalSize", skip_serializing_if = "Option::is_none")]
    journal_size: Option<usize>,
    /// New schema
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<String>,
}

#[cfg(test)]
impl Default for PutPropertiesResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            wait_for_sync: true,
            journal_size: Some(12000000),
            schema: None,
        }
    }
}

/// recalculate count collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct RecalculateCountResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The result
    result: bool,
    /// The new count
    count: usize,
}

#[cfg(test)]
impl Default for RecalculateCountResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            result: true,
            count: 10,
        }
    }
}

/// rename collection response
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct RenameResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
    /// The new name
    name: String,
}

#[cfg(test)]
impl Default for RenameResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
            name: "test_boll".to_string(),
        }
    }
}

/// A new collection
#[derive(Builder, Clone, Debug, Setters, Serialize)]
pub struct NewName {
    /// A new collection name
    #[builder(setter(into))]
    name: String,
}

/// rename collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct TruncateResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
}

#[cfg(test)]
impl Default for TruncateResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
        }
    }
}

/// unload collection response
#[derive(Clone, Copy, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct UnloadResponse {
    /// Is this respone an error?
    error: bool,
    /// The response code, i.e. 200, 404
    code: usize,
}

#[cfg(test)]
impl Default for UnloadResponse {
    fn default() -> Self {
        Self {
            error: false,
            code: 200,
        }
    }
}
