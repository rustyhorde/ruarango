// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Create Input Structs

use crate::{
    model::{
        add_qp, BuildUrl,
        QueryParam::{ReturnNew, WaitForSync},
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

/// Graph create configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The graph to define the new edge in
    #[builder(setter(into))]
    graph: String,
    /// The edge collection to define the new edge in
    #[builder(setter(into))]
    collection: String,
    /// Define if the request should wait until synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Define if the response should contain the complete new
    /// version of the document.
    #[builder(setter(strip_option), default)]
    return_new: Option<bool>,
    /// The from/to mapping for the edge
    mapping: FromTo,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/edge/{}", base, self.graph, self.collection);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        add_qp(*self.return_new(), &mut url, &mut has_qp, ReturnNew);

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

/// The from/to mapping for an edge
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct FromTo {
    /// The from document in a vertex collection
    #[builder(setter(into))]
    #[serde(rename = "_from")]
    from: String,
    /// The to document in a vertex collection
    #[builder(setter(into))]
    #[serde(rename = "_to")]
    to: String,
}
