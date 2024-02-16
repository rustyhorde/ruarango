// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Create Edge Defs Input Structs

use crate::{
    model::{graph::EdgeDefinition, BuildUrl},
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Graph create edge defs configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The graph to create the edge definitions in
    #[builder(setter(into))]
    graph: String,
    /// The edge definition
    edge_def: EdgeDefinition,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        format!("{}/{}/edge", base, self.graph)
    }
}

impl BuildUrl for Config {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{suffix}' url"))
    }
}
