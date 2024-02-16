// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Create Vertex Input Structs

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
use serde::{Deserialize, Serialize};

/// Graph create vertex configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The name of the graph to add the vertex to
    #[builder(setter(into))]
    name: String,
    /// The name of the collection to add the vertex to
    #[builder(setter(into))]
    collection: String,
    /// Wait until the graph has been synced to disk
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,
    /// Return the new vertex in the response
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    return_new: Option<bool>,
    /// The vertex to add
    vertex: T,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/vertex/{}", base, self.name, self.collection);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        add_qp(*self.return_new(), &mut url, &mut has_qp, ReturnNew);

        url
    }
}

impl<T> BuildUrl for Config<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{suffix}' url"))
    }
}
