// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Deletes Input Structs

use crate::{
    model::{
        add_qp, BuildUrl,
        QueryParam::{IgnoreRevs, ReturnOld, WaitForSync},
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Document deletes configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The document keys to delete
    documents: Vec<T>,
    /// Wait until the delete operation has been synced to disk.
    #[builder(setter(strip_option), default)]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete old document under the attribute `old`
    /// in the result.
    #[builder(setter(strip_option), default)]
    return_old: Option<bool>,
    /// If set to true, ignore any `_rev` attribute in the selectors. No
    /// revision check is performed. If set to false then revisions are checked.
    /// The default is true.
    #[builder(setter(into, strip_option), default)]
    ignore_revs: Option<bool>,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection);
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);
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
            .with_context(|| format!("Unable to build '{suffix}' url"))
    }
}

#[cfg(test)]
mod test {
    use super::{Config, ConfigBuilder};
    use crate::model::{
        doc::BASE_DOC_SUFFIX, IGNORE_REVS_QP, RETURN_OLD_QP, TEST_COLL, TEST_KEY, WAIT_FOR_SYNC_QP,
    };
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL);
    const WAIT_FOR_SYNC_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", WAIT_FOR_SYNC_QP);
    const RETURN_OLD_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", RETURN_OLD_QP);
    const IGNORE_REVS_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "?", IGNORE_REVS_QP);
    const ALL_ACTUAL: &str = concatcp!(
        BASIC_ACTUAL,
        "?",
        WAIT_FOR_SYNC_QP,
        "&",
        RETURN_OLD_QP,
        "&",
        IGNORE_REVS_QP
    );

    fn check_url<T>(config: &Config<T>, actual: &str) {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
    }

    #[test]
    fn deletes_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .build()?;
        check_url(&config, BASIC_ACTUAL);
        Ok(())
    }

    #[test]
    fn deletes_wait_for_sync_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .wait_for_sync(true)
            .build()?;
        check_url(&config, WAIT_FOR_SYNC_ACTUAL);
        Ok(())
    }

    #[test]
    fn deletes_return_old_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .return_old(true)
            .build()?;
        check_url(&config, RETURN_OLD_ACTUAL);
        Ok(())
    }

    #[test]
    fn deletes_ignore_revs_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .ignore_revs(true)
            .build()?;
        check_url(&config, IGNORE_REVS_ACTUAL);
        Ok(())
    }

    #[test]
    fn deletes_all() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .wait_for_sync(true)
            .ignore_revs(true)
            .return_old(true)
            .build()?;
        check_url(&config, ALL_ACTUAL);
        Ok(())
    }
}
