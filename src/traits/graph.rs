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
        input::{
            CreateConfig, CreateEdgeDefConfig, CreateVertexCollConfig, DeleteConfig,
            DeleteEdgeDefConfig, EdgeCreateConfig, EdgeDeleteConfig, EdgeReadConfig,
            EdgeReplaceConfig, EdgeUpdateConfig, ReadConfig, ReadEdgeDefsConfig,
            ReadVertexCollsConfig, ReplaceEdgeDefConfig,
        },
        output::{
            CreateEdge, DeleteEdge, EdgesMeta, GraphMeta, List, ReadEdge, ReplaceEdge, UpdateEdge,
            VertexColls,
        },
    },
    ArangoResult,
};
use async_trait::async_trait;
use serde::Serialize;

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
    /// Create an edge definition
    async fn create_edge_def(&self, config: CreateEdgeDefConfig) -> ArangoResult<GraphMeta>;
    /// Read the edge definitions for the given graph
    async fn read_edge_defs(&self, config: ReadEdgeDefsConfig) -> ArangoResult<EdgesMeta>;
    /// Delete an edge definition
    async fn delete_edge_def(&self, config: DeleteEdgeDefConfig) -> ArangoResult<GraphMeta>;
    /// Replace an edge definition
    async fn replace_edge_def(&self, config: ReplaceEdgeDefConfig) -> ArangoResult<GraphMeta>;
    /// Create an edge for a graph
    async fn create_edge(&self, config: EdgeCreateConfig) -> ArangoResult<CreateEdge>;
    /// Delete an edge from a graph
    async fn delete_edge(&self, config: EdgeDeleteConfig) -> ArangoResult<DeleteEdge>;
    /// Read an edge from a graph
    async fn read_edge(&self, config: EdgeReadConfig) -> ArangoResult<ReadEdge>;
    /// Update an edge from a graph
    async fn update_edge<T>(&self, config: EdgeUpdateConfig<T>) -> ArangoResult<UpdateEdge>
    where
        T: Serialize + Send + Sync;
    /// Replace an edge from a graph
    async fn replace_edge<T>(&self, config: EdgeReplaceConfig<T>) -> ArangoResult<ReplaceEdge>
    where
        T: Serialize + Send + Sync;

    /// Read the vertex collections from a graph
    async fn read_vertex_colls(&self, config: ReadVertexCollsConfig) -> ArangoResult<VertexColls>;
    /// Create vertex collection
    async fn create_vertex_coll(&self, config: CreateVertexCollConfig) -> ArangoResult<GraphMeta>;
}
