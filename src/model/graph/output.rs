// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Output Structs

use super::EdgeDefinition;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

/// Output for [`list`](crate::Graph::list)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct List {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// An array of graph data
    graphs: Vec<Graph>,
}

/// Output for [`create`](crate::Graph::create), [`read`](crate::Graph::read)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct GraphMeta {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// An array of graph data
    graph: Graph,
}

/// Graph data
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Graph {
    /// Contains the graph identifier
    #[serde(rename = "_id")]
    id: String,
    /// Contains the graph key
    #[serde(rename = "_key")]
    key: String,
    /// Contains the graph revision
    #[serde(rename = "_rev")]
    rev: String,
    /// The graph name
    name: String,
    /// An array of additional vertex collections.
    /// Documents within these collections do not have edges within this graph.
    #[serde(rename = "orphanCollections")]
    orphan_collections: Vec<String>,
    /// An array of definitions for the relations of the graph.
    #[serde(rename = "edgeDefinitions")]
    edge_definitions: Vec<EdgeDefinition>,
}

/// Output for [`list_edges`](crate::Graph::list_edges)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct EdgeMeta {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The list of all vertex collections within this graph.
    /// Includes collections in edge definitions as well as orphans.
    collections: Vec<String>,
}
