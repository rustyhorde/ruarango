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

/// Output for [`read_edge_defs`](crate::Graph::read_edge_defs)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct EdgesMeta {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The list of all vertex collections within this graph.
    /// Includes collections in edge definitions as well as orphans.
    collections: Vec<String>,
}

/// Output for [`create_edge`](crate::Graph::create_edge)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct CreateEdge {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The edge meta
    edge: EdgeMeta,
    /// The new edge meta
    #[serde(skip_serializing_if = "Option::is_none")]
    new: Option<EdgeMeta>,
}

/// Edge meta data
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct EdgeMeta {
    /// Contains the graph identifier
    #[serde(rename = "_id")]
    id: String,
    /// Contains the graph key
    #[serde(rename = "_key")]
    key: String,
    /// Contains the graph revision
    #[serde(rename = "_rev")]
    rev: String,
    /// Contains the graph revision
    #[serde(rename = "_from", skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// Contains the graph revision
    #[serde(rename = "_to", skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

/// Output for [`delete_edge`](crate::Graph::delete_edge)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct DeleteEdge {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// If set to true, the delete was successful
    removed: bool,
    /// The old edge meta
    #[serde(skip_serializing_if = "Option::is_none")]
    old: Option<EdgeMeta>,
}

/// Output for [`read_edge`](crate::Graph::read_edge)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct ReadEdge {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The edge meta
    edge: EdgeMeta,
}

/// Output for [`update_edge`](crate::Graph::update_edge)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct UpdateEdge {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The edge meta
    edge: EdgeMeta,
    /// The old edge meta
    old: Option<EdgeMeta>,
    /// The new edge meta
    new: Option<EdgeMeta>,
}

/// Output for [`replace_edge`](crate::Graph::replace_edge)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct ReplaceEdge {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The edge meta
    edge: EdgeMeta,
    /// The old edge meta
    old: Option<EdgeMeta>,
    /// The new edge meta
    new: Option<EdgeMeta>,
}

/// Output for [`read_vertex_colls`](crate::Graph::read_vertex_colls)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct VertexColls {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The list of vertex collections
    collections: Vec<String>,
}

/// Output for [`create_vertex`](crate::Graph::create_vertex)
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct VertexMeta {
    /// A flag to indicate that an error occurred
    error: bool,
    /// The HTTP repsponse code
    code: u16,
    /// The vertex data
    vertex: Vertex,
    /// Optional new vertex data
    new: Option<Vertex>,
}

/// Vertex data
#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Vertex {
    /// Contains the graph identifier
    #[serde(rename = "_id")]
    id: String,
    /// Contains the graph key
    #[serde(rename = "_key")]
    key: String,
    /// Contains the graph revision
    #[serde(rename = "_rev")]
    rev: String,
}