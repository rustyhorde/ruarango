// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Update Input Structs

use crate::{
    error::RuarangoErr::Unreachable,
    model::{
        add_qp, AddHeaders, BuildUrl,
        QueryParam::{
            IgnoreRevs, KeepNull, MergeObjects, ReturnNew, ReturnOld, Silent, WaitForSync,
        },
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

/// Document update configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The _key of the document to replace
    #[builder(setter(into))]
    key: String,
    /// The patch document
    document: T,
    /// Wait until document has been synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete new document under the attribute `new`
    /// in the result.
    #[builder(setter(strip_option), default)]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute `old`
    /// in the result. Only available if the `overwrite` option is used.
    #[builder(setter(strip_option), default)]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response. No meta-data
    /// will be returned for the created document. This option can be used to
    /// save some network traffic.
    #[builder(setter(strip_option), default)]
    silent: Option<bool>,
    /// If the intention is to delete existing attributes with the update-insert
    /// command, `keep_null` can be used with a value of false.
    /// This will modify the behavior of `create` to remove any attributes from
    /// the existing document that are contained in the patch document
    /// with an attribute value of `null`.
    /// This option controls the update-insert behavior only.
    #[builder(setter(strip_option), default)]
    keep_null: Option<bool>,
    /// Controls whether objects (not arrays) will be merged if present in both the
    /// existing and the update-insert document. If set to false, the value in the
    /// patch document will overwrite the existing document's value. If set to true,
    /// objects will be merged. The default is true.
    /// This option controls the update-insert behavior only.
    #[builder(setter(strip_option), default)]
    merge_objects: Option<bool>,
    /// By default, or if this is set to true, the _rev attributes in
    /// the given document is ignored. If this is set to false, then
    /// the _rev attribute given in the body document is taken as a
    /// precondition. The document is only updated if the current revision
    /// is the one specified.
    #[builder(setter(strip_option), default)]
    ignore_revs: Option<bool>,
    /// You can conditionally replace a document based on a target `rev` by
    /// using the `if_match` option
    #[builder(setter(into, strip_option), default)]
    if_match: Option<String>,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/{}", base, self.collection, self.key);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);

        if self.silent().is_some() {
            add_qp(*self.silent(), &mut url, &mut has_qp, Silent);
        } else {
            add_qp(*self.return_new(), &mut url, &mut has_qp, ReturnNew);
            add_qp(*self.return_old(), &mut url, &mut has_qp, ReturnOld);
        }

        add_qp(*self.keep_null(), &mut url, &mut has_qp, KeepNull);
        add_qp(*self.merge_objects(), &mut url, &mut has_qp, MergeObjects);
        add_qp(*self.ignore_revs(), &mut url, &mut has_qp, IgnoreRevs);

        url
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

impl<T> BuildUrl for Config<T> {
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
        doc::BASE_DOC_SUFFIX, AddHeaders, IGNORE_REVS_QP, KEEP_NULL_FALSE_QP, KEEP_NULL_QP,
        MERGE_OBJECTS_QP, RETURN_NEW_QP, RETURN_OLD_QP, SILENT_QP, TEST_COLL, TEST_KEY,
        WAIT_FOR_SYNC_QP,
    };
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL, "/", TEST_KEY);
    const WAIT_FOR_SYNC_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP);
    const SILENT_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", SILENT_QP);
    const RETURN_NEW_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_NEW_QP);
    const RETURN_OLD_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_OLD_QP);
    const KEEP_NULL_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", KEEP_NULL_QP);
    const KEEP_NULL_FALSE_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", KEEP_NULL_FALSE_QP);
    const MERGE_OBJECTS_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", MERGE_OBJECTS_QP);
    const IGNORE_REVS_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", IGNORE_REVS_QP);
    const WAIT_SILENT_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", SILENT_QP);
    const WAIT_RETURN_OLD_ACTUAL: &str =
        concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", RETURN_OLD_QP);
    const WAIT_RETURN_NEW_ACTUAL: &str =
        concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", RETURN_NEW_QP);
    const ALL_WITH_SILENT_ACTUAL: &str = concatcp!(
        BASIC_ACTUAL,
        "?",
        WAIT_FOR_SYNC_QP,
        "&",
        SILENT_QP,
        "&",
        KEEP_NULL_QP,
        "&",
        MERGE_OBJECTS_QP,
        "&",
        IGNORE_REVS_QP
    );
    const ALL_WITH_RETURNS_ACTUAL: &str = concatcp!(
        BASIC_ACTUAL,
        "?",
        WAIT_FOR_SYNC_QP,
        "&",
        RETURN_NEW_QP,
        "&",
        RETURN_OLD_QP,
        "&",
        KEEP_NULL_QP,
        "&",
        MERGE_OBJECTS_QP,
        "&",
        IGNORE_REVS_QP
    );

    fn check_url<T>(config: &Config<T>, actual: &str) {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
    }

    #[test]
    fn update_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .build()?;
        check_url(&config, BASIC_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_wait_for_sync_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .build()?;
        check_url(&config, WAIT_FOR_SYNC_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_silent_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .silent(true)
            .build()?;
        check_url(&config, SILENT_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_silent_forces_no_return_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .silent(true)
            .return_old(true)
            .return_new(true)
            .build()?;
        check_url(&config, SILENT_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_return_old_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .return_old(true)
            .build()?;
        check_url(&config, RETURN_OLD_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_return_new_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .return_new(true)
            .build()?;
        check_url(&config, RETURN_NEW_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_keep_null_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .keep_null(true)
            .build()?;
        check_url(&config, KEEP_NULL_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_keep_null_false_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .keep_null(false)
            .build()?;
        check_url(&config, KEEP_NULL_FALSE_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_merge_objects_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .merge_objects(true)
            .build()?;
        check_url(&config, MERGE_OBJECTS_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_ignore_revs_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test)")
            .ignore_revs(true)
            .build()?;
        check_url(&config, IGNORE_REVS_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_wait_silent() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .silent(true)
            .build()?;
        check_url(&config, WAIT_SILENT_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_wait_return_old() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .return_old(true)
            .build()?;
        check_url(&config, WAIT_RETURN_OLD_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_wait_return_new() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .return_new(true)
            .build()?;
        check_url(&config, WAIT_RETURN_NEW_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_all_silent() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .silent(true)
            .return_new(true)
            .return_old(true)
            .keep_null(true)
            .merge_objects(true)
            .ignore_revs(true)
            .build()?;
        check_url(&config, ALL_WITH_SILENT_ACTUAL);
        Ok(())
    }

    #[test]
    fn update_all_returns() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .wait_for_sync(true)
            .return_new(true)
            .return_old(true)
            .keep_null(true)
            .merge_objects(true)
            .ignore_revs(true)
            .build()?;
        check_url(&config, ALL_WITH_RETURNS_ACTUAL);
        Ok(())
    }

    #[test]
    fn has_header() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .key(TEST_KEY)
            .document("test")
            .if_match("_rev")
            .build()?;
        let headers_opt = config.add_headers()?;
        assert!(headers_opt.is_some());
        assert_eq!(headers_opt.unwrap().keys_len(), 1);
        Ok(())
    }
}
