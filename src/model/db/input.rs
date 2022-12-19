// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Database Input Structs
//!
//! # Example
//! ```
//! # use anyhow::Result;
//! # use ruarango::db::input::{CreateBuilder, OptionsBuilder, UserBuilder};
//! #
//! # pub fn main() -> Result<()> {
//! // Use the default options
//! let options = OptionsBuilder::default().build()?;
//!
//! // Configure an active user authorized for this database
//! let user = UserBuilder::default()
//!     .username("test")
//!     .password("test")
//!     .active(true)
//!     .build()?;
//!
//! // Setup the final `Create` configuration
//! let create = CreateBuilder::default()
//!     .name("test_db")
//!     .options(options)
//!     .users(vec![user])
//!     .build()?;
//!
//! // The name field is required
//! assert!(CreateBuilder::default().build().is_err());
//!
//! // The `Options` and `User` configuration are optional
//! let base = CreateBuilder::default().name("test_db_1").build()?;
//! #   Ok(())
//! # }
//! ```
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Database creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
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

/// Optional clustering configuration used during database creation
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
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

/// Optional user information used during database creation
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
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

#[cfg(test)]
mod test {
    use super::CreateBuilder;

    #[test]
    fn current_builder_fails_when_missing_name() {
        assert!(CreateBuilder::default().build().is_err());
    }
}
