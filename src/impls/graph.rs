// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph trait implementation

use super::EMPTY_BODY;
use crate::{
    cursor::BASE_CURSOR_SUFFIX,
    graph::{
        input::{
            CreateConfig, CreateEdgeDefConfig, CreateVertexCollConfig, CreateVertexConfig,
            DeleteConfig, DeleteEdgeDefConfig, DeleteVertexCollConfig, DeleteVertexConfig,
            EdgeCreateConfig, EdgeDeleteConfig, EdgeReadConfig, EdgeReplaceConfig,
            EdgeUpdateConfig, ReadConfig, ReadEdgeDefsConfig, ReadVertexCollsConfig,
            ReadVertexConfig, ReplaceEdgeDefConfig, UpdateVertexConfig,
        },
        output::{
            CreateEdge, DeleteEdge, DeleteVertexMeta, EdgesMeta, GraphMeta, List, ReadEdge,
            ReadVertexMeta, ReplaceEdge, UpdateEdge, UpdateVertexMeta, VertexColls, VertexMeta,
        },
        BASE_GRAPH_SUFFIX,
    },
    model::{AddHeaders, BuildUrl},
    traits::Graph,
    utils::{empty, handle_response, map_resp},
    ArangoResult, Connection,
};
use anyhow::Context;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
#[allow(unused_qualifications)]
impl Graph for Connection {
    async fn list(&self) -> ArangoResult<List> {
        let url = self
            .db_url()
            .join(BASE_GRAPH_SUFFIX)
            .with_context(|| format!("Unable to build '{BASE_CURSOR_SUFFIX}' url"))?;
        self.get(url, None, EMPTY_BODY, handle_response).await
    }

    async fn create(&self, config: CreateConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.graph(), handle_response).await
    }

    async fn read(&self, config: ReadConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.get(url, None, EMPTY_BODY, handle_response).await
    }
    async fn delete(&self, config: DeleteConfig) -> ArangoResult<()> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.delete(url, None, EMPTY_BODY, empty).await
    }

    async fn create_edge_def(&self, config: CreateEdgeDefConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.edge_def(), handle_response)
            .await
    }

    async fn read_edge_defs(&self, config: ReadEdgeDefsConfig) -> ArangoResult<EdgesMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.get(url, None, EMPTY_BODY, handle_response).await
    }

    async fn delete_edge_def(&self, config: DeleteEdgeDefConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.delete(url, None, EMPTY_BODY, handle_response).await
    }

    async fn replace_edge_def(&self, config: ReplaceEdgeDefConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.put(url, None, config.edge_def(), handle_response)
            .await
    }

    async fn create_edge(&self, config: EdgeCreateConfig) -> ArangoResult<CreateEdge> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.mapping(), handle_response)
            .await
    }

    async fn delete_edge(&self, config: EdgeDeleteConfig) -> ArangoResult<DeleteEdge> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.delete(url, headers, EMPTY_BODY, handle_response).await
    }

    async fn read_edge(&self, config: EdgeReadConfig) -> ArangoResult<ReadEdge> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.get(url, headers, EMPTY_BODY, handle_response).await
    }

    async fn update_edge<T>(&self, config: EdgeUpdateConfig<T>) -> ArangoResult<UpdateEdge>
    where
        T: Serialize + Send + Sync,
    {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.patch(url, headers, config.edge(), handle_response)
            .await
    }

    async fn replace_edge<T>(&self, config: EdgeReplaceConfig<T>) -> ArangoResult<ReplaceEdge>
    where
        T: Serialize + Send + Sync,
    {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.put(url, headers, config.edge(), handle_response).await
    }

    async fn read_vertex_colls(&self, config: ReadVertexCollsConfig) -> ArangoResult<VertexColls> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.get(url, None, EMPTY_BODY, handle_response).await
    }

    async fn create_vertex_coll(&self, config: CreateVertexCollConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.collection(), map_resp).await
    }

    async fn delete_vertex_coll(&self, config: DeleteVertexCollConfig) -> ArangoResult<GraphMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.delete(url, None, EMPTY_BODY, map_resp).await
    }

    async fn create_vertex<T>(&self, config: CreateVertexConfig<T>) -> ArangoResult<VertexMeta>
    where
        T: Serialize + Send + Sync,
    {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.vertex(), map_resp).await
    }

    async fn delete_vertex(&self, config: DeleteVertexConfig) -> ArangoResult<DeleteVertexMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.delete(url, headers, EMPTY_BODY, map_resp).await
    }

    async fn read_vertex(&self, config: ReadVertexConfig) -> ArangoResult<ReadVertexMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.get(url, headers, EMPTY_BODY, map_resp).await
    }

    async fn update_vertex<T>(
        &self,
        config: UpdateVertexConfig<T>,
    ) -> ArangoResult<UpdateVertexMeta>
    where
        T: Serialize + Send + Sync,
    {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.patch(url, headers, config.vertex(), map_resp).await
    }

    async fn replace_vertex<T>(
        &self,
        config: UpdateVertexConfig<T>,
    ) -> ArangoResult<UpdateVertexMeta>
    where
        T: Serialize + Send + Sync,
    {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        let headers = config.add_headers()?;
        self.put(url, headers, config.vertex(), map_resp).await
    }
}
