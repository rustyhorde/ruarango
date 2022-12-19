// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Create Input Structs

use crate::{
    model::{add_qp, graph::EdgeDefinition, BuildUrl, QueryParam::WaitForSync},
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

const EMPTY_EDGE_DEFINITIONS_ERR: &str = "edge_definitions cannot be empty!";

/// Graph create configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The graph meta to load
    graph: GraphMeta,
    /// Wait until the graph has been synced to disk.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = base.to_string();
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);

        url
    }
}

impl BuildUrl for Config {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Document creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
#[builder(build_fn(validate = "Self::validate"))]
pub struct GraphMeta {
    /// The name of the graph to create
    #[builder(setter(into))]
    name: String,
    /// An array of definitions for the relations of the graph.
    #[serde(rename = "edgeDefinitions")]
    edge_definitions: Vec<EdgeDefinition>,
    /// An array of additional vertex collections.
    /// Documents within these collections do not have edges within this graph.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    orphan_collections: Option<Vec<String>>,
}

impl GraphMetaBuilder {
    fn validate(&self) -> std::result::Result<(), String> {
        self.edge_definitions.as_ref().map_or(Ok(()), |ed| {
            if ed.is_empty() {
                Err(EMPTY_EDGE_DEFINITIONS_ERR.into())
            } else {
                Ok(())
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Config, ConfigBuilder, GraphMetaBuilder, EMPTY_EDGE_DEFINITIONS_ERR};
    use crate::{
        graph::BASE_GRAPH_SUFFIX,
        model::{
            graph::{EdgeDefinition, EdgeDefinitionBuilder},
            WAIT_FOR_SYNC_QP,
        },
    };
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_GRAPH_SUFFIX);
    const WAIT_FOR_SYNC_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP);

    fn check_url(config: Config, actual: &str) -> Result<()> {
        assert_eq!(actual, config.build_suffix(BASE_GRAPH_SUFFIX));
        Ok(())
    }

    fn ve(vec: Vec<&str>) -> Vec<String> {
        vec.into_iter().map(str::to_string).collect()
    }

    fn edge_definition() -> Result<Vec<EdgeDefinition>> {
        let ed = EdgeDefinitionBuilder::default()
            .collection("test_edge")
            .from(ve(vec!["test_coll"]))
            .to(ve(vec!["test_coll"]))
            .build()?;
        Ok(vec![ed])
    }

    #[test]
    fn create_url() -> Result<()> {
        let graph_meta = GraphMetaBuilder::default()
            .name("test")
            .edge_definitions(edge_definition()?)
            .build()?;
        let config = ConfigBuilder::default().graph(graph_meta).build()?;
        check_url(config, BASIC_ACTUAL)
    }

    #[test]
    fn create_wait_for_sync_url() -> Result<()> {
        let graph_meta = GraphMetaBuilder::default()
            .name("test")
            .edge_definitions(edge_definition()?)
            .build()?;
        let config = ConfigBuilder::default()
            .graph(graph_meta)
            .wait_for_sync(true)
            .build()?;
        check_url(config, WAIT_FOR_SYNC_ACTUAL)
    }

    #[test]
    fn empty_ed_errors() -> Result<()> {
        match GraphMetaBuilder::default()
            .name("test")
            .edge_definitions(vec![])
            .build()
        {
            Ok(_) => panic!("The builder should fail!"),
            Err(e) => assert_eq!(EMPTY_EDGE_DEFINITIONS_ERR, format!("{}", e)),
        }
        Ok(())
    }
}
