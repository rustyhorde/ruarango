// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Database Output Structs

use getset::Getters;
use serde::{Deserialize, Serialize};

/// Output when [`current`](crate::Database::current) is called for a document
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
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
