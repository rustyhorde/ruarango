// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database response models

use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

/// Create
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Create {
    /// A valid database name
    #[builder(setter(into))]
    name: String,
    /// Optional database options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    options: Option<Options>,
    /// Optional array of users to initially create for the new database.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    users: Option<Vec<User>>,
}

/// Optional clustering options used during database creation
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Options {
    /// The sharding method to use for new collections in this database. Valid values are: "", "flexible", or "single".
    /// The first two are equivalent. (cluster only)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    sharding: Option<String>,
    /// Default replication factor for new collections created in this database.
    /// Special values include "satellite", which will replicate the collection
    /// to every DB-Server (Enterprise Edition only), and 1, which disables replication (cluster only)
    #[serde(rename = "replicationFactor", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    replication_factor: Option<String>,
    /// Default write concern for new collections created in this database.
    /// It determines how many copies of each shard are required to be
    /// in sync on the different DB-Servers. If there are less then these many copies
    /// in the cluster a shard will refuse to write. Writes to shards with enough
    /// up-to-date copies will succeed at the same time however. The value of
    /// writeConcern can not be larger than replicationFactor. (cluster only)
    #[serde(rename = "writeConcern", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    write_concern: Option<String>,
}

/// Optional user information for database creation
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct User {
    /// Login name of the user to be created.
    #[builder(setter(into))]
    username: String,
    /// The user password as a string. If not specified, it will default to an empty string.
    #[builder(setter(into))]
    password: String,
    /// A flag indicating whether the user account should be activated or not.
    /// The default value is true. If set to false, the user won't be able to
    /// log into the database.
    active: bool,
}

/// Response for the `_api/database/current` endpoint
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct Current {
    /// The name of the current database
    name: String,
    /// The id of the current database
    id: String,
    /// Is the current database the `_system` database
    #[serde(rename = "isSystem")]
    is_system: bool,
    /// The filesystem path of the current database
    path: String,
    /// The default sharding method for collections created in this database
    #[serde(skip_serializing_if = "Option::is_none")]
    sharding: Option<String>,
    /// The default replication factor for collections in this database
    #[serde(rename = "replicationFactor", skip_serializing_if = "Option::is_none")]
    replication_factor: Option<String>,
    /// The default write concern for collections in this database
    #[serde(rename = "writeConcern", skip_serializing_if = "Option::is_none")]
    write_concern: Option<String>,
}

#[cfg(test)]
impl Default for Current {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            id: "123".to_string(),
            is_system: false,
            path: "abcdef".to_string(),
            sharding: None,
            replication_factor: None,
            write_concern: None,
        }
    }
}
