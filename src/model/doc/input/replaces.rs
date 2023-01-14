// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Replaces Input Structs

use crate::{
    model::{
        add_qp, BuildUrl,
        QueryParam::{IgnoreRevs, ReturnNew, ReturnOld, WaitForSync},
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Document replace configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The patch documents
    documents: Vec<T>,
    /// Wait until the delete operation has been synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// By default, or if this is set to true, the `_rev` attribute in
    /// the given document is ignored. If this is set to false, then
    /// the `_rev` attribute given in the body document is taken as a
    /// precondition. The document is only replaced if the current revision
    /// is the one specified.
    #[builder(setter(strip_option), default)]
    ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute `new`
    /// in the result.
    #[builder(setter(strip_option), default)]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute `old`
    /// in the result.
    #[builder(setter(strip_option), default)]
    return_old: Option<bool>,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
        add_qp(*self.return_new(), &mut url, &mut has_qp, ReturnNew);
        add_qp(*self.return_old(), &mut url, &mut has_qp, ReturnOld);
        add_qp(*self.ignore_revs(), &mut url, &mut has_qp, IgnoreRevs);

        url
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
        doc::BASE_DOC_SUFFIX, RETURN_NEW_QP, RETURN_OLD_QP, TEST_COLL, WAIT_FOR_SYNC_QP,
    };
    use anyhow::Result;
    use const_format::concatcp;
    use lazy_static::lazy_static;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL);
    const WAIT_FOR_SYNC_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP);
    const RETURN_NEW_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_NEW_QP);
    const RETURN_OLD_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_OLD_QP);
    const WAIT_RETURN_OLD_ACTUAL: &str =
        concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", RETURN_OLD_QP);
    const WAIT_RETURN_NEW_ACTUAL: &str =
        concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP, "&", RETURN_NEW_QP);
    const WAIT_RETURNS_ACTUAL: &str = concatcp!(
        BASIC_ACTUAL,
        "?",
        WAIT_FOR_SYNC_QP,
        "&",
        RETURN_NEW_QP,
        "&",
        RETURN_OLD_QP
    );

    lazy_static! {
        static ref DOCS: Vec<&'static str> = vec!["test"];
    }

    fn check_url<T>(config: &Config<T>, actual: &str) {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
    }

    #[test]
    fn replaces_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .build()?;
        check_url(&config, BASIC_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_wait_for_sync_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .wait_for_sync(true)
            .build()?;
        check_url(&config, WAIT_FOR_SYNC_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_return_old_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .return_old(true)
            .build()?;
        check_url(&config, RETURN_OLD_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_return_new_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .return_new(true)
            .build()?;
        check_url(&config, RETURN_NEW_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_wait_return_old() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .wait_for_sync(true)
            .return_old(true)
            .build()?;
        check_url(&config, WAIT_RETURN_OLD_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_wait_return_new() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .wait_for_sync(true)
            .return_new(true)
            .build()?;
        check_url(&config, WAIT_RETURN_NEW_ACTUAL);
        Ok(())
    }

    #[test]
    fn replaces_wait_returns() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents((*DOCS).clone())
            .wait_for_sync(true)
            .return_old(true)
            .return_new(true)
            .build()?;
        check_url(&config, WAIT_RETURNS_ACTUAL);
        Ok(())
    }
}
