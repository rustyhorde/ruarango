// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Graph Update Vertex Input Structs

use crate::{
    error::RuarangoErr::Unreachable,
    model::{
        add_qp, AddHeaders, BuildUrl,
        QueryParam::{KeepNull, ReturnNew, ReturnOld, WaitForSync},
    },
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

/// Graph update vertex configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The name of the graph where the vertex to update is
    #[builder(setter(into))]
    name: String,
    /// The name of the collection where the vertex to update is
    #[builder(setter(into))]
    collection: String,
    /// The key of the vertex to update
    #[builder(setter(into))]
    key: String,
    /// Wait until the graph has been synced to disk
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,
    /// Define if values set to null should be stored.
    /// By default (true) the given vertex attribute(s) will be set to null.
    /// If this parameter is false the attribute(s) will instead be deleted from the
    /// vertex.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_null: Option<bool>,
    /// Return the old vertex in the response
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    return_old: Option<bool>,
    /// Return the new vertex in the response
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    return_new: Option<bool>,
    /// The vertex will only be updated
    /// The vertex will only be updated if the vertex has a revision
    /// matching the revision given here
    #[builder(setter(strip_option, into), default)]
    if_match: Option<String>,
    /// The vertex to add
    vertex: T,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!(
            "{}/{}/vertex/{}/{}",
            base, self.name, self.collection, self.key
        );
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        add_qp(*self.keep_null(), &mut url, &mut has_qp, KeepNull);
        add_qp(*self.return_old(), &mut url, &mut has_qp, ReturnOld);
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

impl<T> AddHeaders for Config<T> {
    fn has_header(&self) -> bool {
        self.if_match.is_some()
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
            } else {
                return Err(Unreachable {
                    msg: "'if_match' should be true!".to_string(),
                }
                .into());
            }
        }
        Ok(headers)
    }
}
