// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Edge Read Input Structs

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
    /// The `_key` of the edge to delete
    #[builder(setter(into))]
    key: String,
    /// Must contain a revision.
    /// If this is set a document is only returned if it has exactly
    /// this revision.
    /// Also see `if_match` as an alternative to this.
    #[builder(setter(strip_option, into), default)]
    rev: Option<String>,
    /// If `if_match` is given, then it must contain exactly one
    /// `_rev`. The edge is returned if it has the same revision
    /// as the given `_rev`. Otherwise a HTTP 412 is returned.
    /// As an alternative you can supply `rev`.
    #[builder(setter(strip_option, into), default)]
    if_match: Option<String>,
    /// If `if_none_match` is given, then it must contain exactly
    /// one `_rev`. The edge is returned only if it has a different
    /// revision from the given `_rev`. Otherwise a HTTP 304 is
    /// returned.
    #[builder(setter(strip_option, into), default)]
    if_none_match: Option<String>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        format!(
            "{}/{}/edge/{}/{}",
            base, self.graph, self.collection, self.key
        )
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

impl AddHeaders for Config {
    fn has_header(&self) -> bool {
        self.if_match.is_some() || self.if_none_match.is_some() || self.rev.is_some()
    }

    fn add_headers(&self) -> Result<Option<HeaderMap>> {
        let mut headers = None;

        if self.has_header() {
            let mut headers_map = HeaderMap::new();
            if let Some(rev) = self.rev() {
                let _ =
                    headers_map.append(HeaderName::from_static("rev"), HeaderValue::from_str(rev)?);
                headers = Some(headers_map);
            } else if let Some(rev) = self.if_match() {
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
                    msg: "One of 'if_match', 'if_none_match', or 'rev' should be set!".to_string(),
                }
                .into());
            }
        }
        Ok(headers)
    }
}
