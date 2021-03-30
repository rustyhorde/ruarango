// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` collection response models

use getset::Getters;
use serde_derive::Deserialize;
#[cfg(test)]
use serde_derive::Serialize;

/// Collection response data
#[derive(Clone, Debug, Deserialize, Getters)]
#[cfg_attr(test, derive(Serialize))]
#[getset(get = "pub")]
pub struct Collection {
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
impl Default for Collection {
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
pub struct CollectionInfo {
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
impl Default for CollectionInfo {
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
