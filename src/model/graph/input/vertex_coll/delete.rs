// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Delete Vertex Collection Input Structs

use crate::{
    model::{add_qp, BuildUrl, QueryParam::DropCollection},
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

/// Graph delete vertex collection configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The name of the graph to delete the vertex collection from
    #[builder(setter(into))]
    name: String,
    /// The name of the vertex collection to remove
    #[builder(setter(into))]
    collection: String,
    /// Drop the collection as well.
    /// Collection will only be dropped if it is not used in other graphs.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_collection: Option<bool>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/vertex/{}", base, self.name, self.collection);
        let mut has_qp = false;

        add_qp(
            *self.drop_collection(),
            &mut url,
            &mut has_qp,
            DropCollection,
        );

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
