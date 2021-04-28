// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruarango` graph trait

use crate::{
    graph::{
        input::{CreateConfig, DeleteConfig, ReadConfig},
        output::{GraphMeta, List},
    },
    ArangoResult,
};
use async_trait::async_trait;

/// Database Operations
#[async_trait]
pub trait Graph {
    /// List all graphs
    async fn list(&self) -> ArangoResult<List>;
    /// Create a graph
    async fn create(&self, config: CreateConfig) -> ArangoResult<GraphMeta>;
    /// Read a graph
    async fn read(&self, config: ReadConfig) -> ArangoResult<GraphMeta>;
    /// Delete a graph
    async fn delete(&self, config: DeleteConfig) -> ArangoResult<()>;
}
