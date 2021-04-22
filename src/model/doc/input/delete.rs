// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Delete Input Structs

use crate::{
    error::RuarangoErr::Unreachable,
    model::{
        add_qp, AddHeaders, BuildUrl,
        QueryParam::{ReturnOld, Silent, WaitForSync},
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
use serde_derive::{Deserialize, Serialize};

/// Document delete configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The _key of the document to replace
    #[builder(setter(into))]
    key: String,
    /// Wait until the delete operation has been synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete old document under the attribute `old`
    /// in the result.
    #[builder(setter(strip_option), default)]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response. No meta-data
    /// will be returned for the deleted document. This option can be used to
    /// save some network traffic.
    #[builder(setter(strip_option), default)]
    silent: Option<bool>,
    /// You can conditionally remove a document based on a target `rev` by
    /// using the `if_match` option
    #[builder(setter(into, strip_option), default)]
    if_match: Option<String>,
}

impl Config {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/{}", base, self.collection, self.key);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        if self.silent().is_some() {
            add_qp(*self.silent(), &mut url, &mut has_qp, Silent);
        } else {
            add_qp(*self.return_old(), &mut url, &mut has_qp, ReturnOld);
        }

        url
    }
}

impl AddHeaders for Config {
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

impl BuildUrl for Config {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

#[cfg(test)]
mod test {
    use super::{Config, ConfigBuilder};
    use crate::model::{
        doc::BASE_DOC_SUFFIX, AddHeaders, RETURN_OLD_QP, SILENT_QP, TEST_COLL, TEST_KEY,
        WAIT_FOR_SYNC_QP,
    };
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL, "/", TEST_KEY);
    const WAIT_FOR_SYNC_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP);
    const SILENT_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", SILENT_QP);
    const RETURN_OLD_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_OLD_QP);
    const WAIT_RETURN_ACTUAL: &str =
        concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", RETURN_OLD_QP);
    const WAIT_SILENT_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", SILENT_QP);

    fn check_url(config: Config, actual: &str) -> Result<()> {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
        Ok(())
    }

    #[test]
    fn delete_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .build()?;
        check_url(config, BASIC_ACTUAL)
    }

    #[test]
    fn delete_wait_for_sync_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .wait_for_sync(true)
            .build()?;
        check_url(config, WAIT_FOR_SYNC_ACTUAL)
    }

    #[test]
    fn delete_silent_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .silent(true)
            .build()?;
        check_url(config, SILENT_ACTUAL)
    }

    #[test]
    fn delete_silent_forces_no_return_old_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .silent(true)
            .return_old(true)
            .build()?;
        check_url(config, SILENT_ACTUAL)
    }

    #[test]
    fn delete_return_old_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .return_old(true)
            .build()?;
        check_url(config, RETURN_OLD_ACTUAL)
    }

    #[test]
    fn delete_wait_return() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .wait_for_sync(true)
            .return_old(true)
            .build()?;
        check_url(config, WAIT_RETURN_ACTUAL)
    }

    #[test]
    fn delete_wait_silent() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .wait_for_sync(true)
            .silent(true)
            .build()?;
        check_url(config, WAIT_SILENT_ACTUAL)
    }

    #[test]
    fn delete_all() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .wait_for_sync(true)
            .silent(true)
            .return_old(true)
            .build()?;
        check_url(config, WAIT_SILENT_ACTUAL)
    }

    #[test]
    fn has_header() -> Result<()> {
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
