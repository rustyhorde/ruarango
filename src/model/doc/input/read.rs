// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Read Input Structs

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

/// Read document configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The collection to read the document from
    #[builder(setter(into))]
    collection: String,
    /// The document _key
    #[builder(setter(into))]
    key: String,
    /// If the `if_none_match` option is given, then it must contain exactly one
    /// revision. The document is returned if it has a different revision than the
    /// given revision. Otherwise, an HTTP 304 is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    if_none_match: Option<String>,
    /// If the `if_match` option is given, then it must contain exactly one
    /// revision. The document is returned if it has the same revision as the
    /// given revision. Otherwise a HTTP 412 is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    if_match: Option<String>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        format!("{}/{}/{}", base, self.collection, self.key)
    }
}

impl BuildUrl for Config {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
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
                    msg: "One of 'if_match' or 'if_none_match' should be true!".to_string(),
                }
                .into());
            }
        }
        Ok(headers)
    }
}

#[cfg(test)]
mod test {
    use super::{Config, ConfigBuilder};
    use crate::model::{doc::BASE_DOC_SUFFIX, AddHeaders, TEST_COLL, TEST_KEY};
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL, "/", TEST_KEY);

    fn check_url(config: Config, actual: &str) -> Result<()> {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
        Ok(())
    }

    #[test]
    fn read_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .build()?;
        check_url(config, BASIC_ACTUAL)
    }

    #[test]
    fn has_if_match_header() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .if_match("_rev")
            .build()?;
        let headers_opt = config.add_headers()?;
        assert!(headers_opt.is_some());
        assert_eq!(headers_opt.unwrap().keys_len(), 1);
        Ok(())
    }

    #[test]
    fn has_if_none_match_header() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .if_none_match("_rev")
            .build()?;
        let headers_opt = config.add_headers()?;
        assert!(headers_opt.is_some());
        assert_eq!(headers_opt.unwrap().keys_len(), 1);
        Ok(())
    }

    #[test]
    fn has_no_header() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .build()?;
        let headers_opt = config.add_headers()?;
        assert!(headers_opt.is_none());
        Ok(())
    }
}
