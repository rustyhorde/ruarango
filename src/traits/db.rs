// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` database trait

use super::Res;
use crate::{
    common::output::Response,
    db::{input::Create, output::Current},
};
use anyhow::Result;
use async_trait::async_trait;

/// Database Operations
#[async_trait]
pub trait Database {
    /// Retrieves the properties of the current database
    async fn current(&self) -> Result<Res<Response<Current>>>;
    /// Retrieves the list of all databases the current user can access without specifying a different username or password.
    async fn user(&self) -> Result<Response<Vec<String>>>;
    /// Retrieves the list of all existing databases
    /// *Note*: retrieving the list of databases is only possible from within the _system database.
    /// *Note*: You should use the `GET user API` to fetch the list of the available databases now.
    async fn list(&self) -> Result<Response<Vec<String>>>;
    /// Creates a new database
    /// *Note*: creating a new database is only possible from within the _system database.
    async fn create(&self, db: &Create) -> Result<Response<bool>>;
    /// Drops the database along with all data stored in it.
    /// *Note*: dropping a database is only possible from within the _system database.
    /// The _system database itself cannot be dropped.
    async fn drop(&self, name: &str) -> Result<Response<bool>>;
}
