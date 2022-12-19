// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Reads Input Structs

use crate::{
    model::{
        add_qp, BuildUrl,
        QueryParam::{IgnoreRevs, OnlyGet},
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Document reads configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The collection to read the documents from
    #[builder(setter(into))]
    collection: String,
    /// Should the value be true (the default):
    /// If a search document contains a value for the `_rev` field,
    /// then the document is only returned if it has the same revision value.
    /// Otherwise a precondition failed error is returned.
    #[builder(setter(strip_option), default)]
    ignore_revs: Option<bool>,
    /// The search documents to read
    documents: Vec<T>,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection);
        let mut has_qp = false;

        add_qp(Some(true), &mut url, &mut has_qp, |_| OnlyGet);
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
    use crate::model::{doc::BASE_DOC_SUFFIX, IGNORE_REVS_QP, ONLYGET_QP, TEST_COLL, TEST_KEY};
    use anyhow::Result;
    use const_format::concatcp;

    const BASIC_ACTUAL: &str = concatcp!(BASE_DOC_SUFFIX, "/", TEST_COLL, "?", ONLYGET_QP);
    const IGNORE_REVS_ACTUAL: &str = concatcp!(BASIC_ACTUAL, "&", IGNORE_REVS_QP);

    fn check_url<T>(config: Config<T>, actual: &str) -> Result<()> {
        assert_eq!(actual, config.build_suffix(BASE_DOC_SUFFIX));
        Ok(())
    }

    #[test]
    fn reads_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .build()?;
        check_url(config, BASIC_ACTUAL)
    }

    #[test]
    fn reads_ignore_revs_url() -> Result<()> {
        let config = ConfigBuilder::default()
            .collection(TEST_COLL)
            .documents(vec![TEST_KEY])
            .ignore_revs(true)
            .build()?;
        check_url(config, IGNORE_REVS_ACTUAL)
    }
}
