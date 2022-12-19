// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Creates Input Structs

use super::OverwriteMode;
use crate::{
    model::{
        add_qp, add_qps, BuildUrl,
        QueryParam::{
            KeepNull, MergeObjects, Overwrite, OverwriteMode as Mode, ReturnNew, ReturnOld, Silent,
            WaitForSync,
        },
    },
    Connection,
};
use anyhow::{Context, Result};
use derive_builder::Builder;
use getset::Getters;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Documents creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct Config<T> {
    /// The collection to create the document in
    #[builder(setter(into))]
    collection: String,
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
    /// If set to true, the insert becomes a replace-insert. If a document with the
    /// same `_key` already exists the new document is not rejected with unique
    /// constraint violated but will replace the old document. Note that operations
    /// with overwrite require a `_key` attribute in the given document.
    /// Therefore, they can only be performed on collections sharded by `_key`.
    #[builder(setter(strip_option), default)]
    overwrite: Option<bool>,
    /// This option supersedes overwrite
    #[builder(setter(strip_option), default)]
    overwrite_mode: Option<OverwriteMode>,
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
    /// The document to create
    document: Vec<T>,
}

impl<T> Config<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection());
        let mut has_qp = false;

        add_qp(*self.wait_for_sync(), &mut url, &mut has_qp, WaitForSync);

        if self.silent().is_some() {
            add_qp(*self.silent(), &mut url, &mut has_qp, Silent);
        } else {
            add_qp(*self.return_new(), &mut url, &mut has_qp, ReturnNew);
            add_qp(*self.return_old(), &mut url, &mut has_qp, ReturnOld);
        }

        if let Some(mode) = self.overwrite_mode() {
            add_qps(*self.overwrite_mode(), &mut url, &mut has_qp, Mode);

            if *mode == OverwriteMode::Update {
                add_qp(*self.keep_null(), &mut url, &mut has_qp, KeepNull);
                add_qp(*self.merge_objects(), &mut url, &mut has_qp, MergeObjects);
            }
        } else if self.overwrite().is_some() {
            add_qp(*self.overwrite(), &mut url, &mut has_qp, Overwrite);
        }

        url
    }
}

impl<T> BuildUrl for Config<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}
