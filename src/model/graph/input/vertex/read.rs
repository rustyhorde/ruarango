// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Read Vertex Input Structs

use crate::{
    error::RuarangoErr::Unreachable,
    model::{AddHeaders, BuildUrl},
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use serde::{Deserialize, Serialize};

/// Graph read vertex configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The name of the graph to read the vertex from
    #[builder(setter(into))]
    name: String,
    /// The name of the collection to read the vertex from
    #[builder(setter(into))]
    collection: String,
    /// The key of the vertex to read
    #[builder(setter(into))]
    key: String,
    /// The vertex will only be read if the vertex has a revision
    /// matching the revision given here
    #[builder(setter(strip_option, into), default)]
    if_match: Option<String>,
    /// The vertex will only be read if the vertex has a revision
    /// not matching the revision given here
    #[builder(setter(strip_option, into), default)]
    if_none_match: Option<String>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        format!(
            "{}/{}/vertex/{}/{}",
            base, self.name, self.collection, self.key
        )
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

impl AddHeaders for Config {
    fn has_header(&self) -> bool {
        self.if_match.is_some() || self.if_none_match.is_some()
    }

    fn add_headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = None;

        if self.has_header() {
            let mut headers_map = HeaderMap::new();
            if let Some(rev) = self.if_match() {
                let _ = headers_map.append(
                    HeaderName::from_static("if-match"),
                    HeaderValue::from_str(rev)?,
                );
                headers = Some(headers_map);
            } else if let Some(rev) = self.if_none_match() {
                let _ = headers_map.append(
                    HeaderName::from_static("if-none-match"),
                    HeaderValue::from_str(rev)?,
                );
                headers = Some(headers_map);
            } else {
                return Err(Unreachable {
                    msg: "'if_match' or 'if_none_match' should be true!".to_string(),
                }
                .into());
            }
        }
        Ok(headers)
    }
}
