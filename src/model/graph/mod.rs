// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [`Input`](crate::graph::input)/[`Output`](crate::graph::output) for [`Graph`](crate::Graph) operations

pub mod input;
pub mod output;

pub(crate) const BASE_GRAPH_SUFFIX: &str = "_api/gharial";

use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// Edge Definition Data
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub")]
pub struct EdgeDefinition {
    /// Name of the edge collection, where the edge are stored in.
    #[builder(setter(into))]
    collection: String,
    /// List of vertex collection names.
    /// Edges in collection can only be inserted if their `_to` is in
    /// any of the collections here.
    #[builder(setter(into))]
    to: Vec<String>,
    /// List of vertex collection names.
    /// Edges in collection can only be inserted if their `_to` is in
    /// any of the collections here.
    from: Vec<String>,
}
