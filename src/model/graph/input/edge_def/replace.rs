// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Replace Edge Def Input Structs

use crate::{
    graph::EdgeDefinition,
    model::{
        add_qp, BuildUrl,
        QueryParam::{DropCollections, WaitForSync},
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Graph replace edge def configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The graph to create the edge definitions in
    #[builder(setter(into))]
    graph: String,
    /// The edge definition name
    #[builder(setter(into))]
    edge_def: EdgeDefinition,
    /// Define if the request should wait until synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Drop the collection as well.
    /// Collection will only be dropped if it is not used in other graphs.
    #[builder(setter(strip_option), default)]
    drop_collections: Option<bool>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!(
            "{}/{}/edge/{}",
            base,
            self.graph,
            self.edge_def.collection()
        );
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        add_qp(
            *self.drop_collections(),
            &mut url,
            &mut has_qp,
            DropCollections,
        );

        url
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
