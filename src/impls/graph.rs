// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph trait implementation

use crate::{
    cursor::BASE_CURSOR_SUFFIX,
    graph::{
        input::{CreateConfig, DeleteConfig, EdgeCreateConfig, ListEdgesConfig, ReadConfig},
        output::{CreateEdge, EdgesMeta, GraphMeta, List},
        BASE_GRAPH_SUFFIX,
    },
    model::BuildUrl,
    traits::Graph,
    utils::{empty, handle_response},
    ArangoResult, Connection,
};
use anyhow::Context;
use async_trait::async_trait;

use super::EMPTY_BODY;

#[async_trait]
impl Graph for Connection {
    async fn list(&self) -> ArangoResult<List> {
        let url = self
            .db_url()
            .join(BASE_GRAPH_SUFFIX)
            .with_context(|| format!("Unable to build '{}' url", BASE_CURSOR_SUFFIX))?;
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

    async fn list_edges(&self, config: ListEdgesConfig) -> ArangoResult<EdgesMeta> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.get(url, None, EMPTY_BODY, handle_response).await
    }

    async fn create_edge(&self, config: EdgeCreateConfig) -> ArangoResult<CreateEdge> {
        let url = config.build_url(BASE_GRAPH_SUFFIX, self)?;
        self.post(url, None, config.mapping(), handle_response)
            .await
    }
}
