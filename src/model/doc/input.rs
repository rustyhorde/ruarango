// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Document Input Structs

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
use serde::{
    de::{self, Deserialize as Deser, Deserializer, Visitor},
    ser::{Serialize as Ser, Serializer},
};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

macro_rules! add_qp {
    ($url:ident, $has_qp:ident, $val:expr;) => {
        let _ = prepend_sep(&mut $url, $has_qp);
        $url += $val;
    };
    ($url:ident, $has_qp:ident, $val:expr) => {
        let _ = prepend_sep(&mut $url, $has_qp);
        $url += $val;
        $has_qp = true;
    };
}

fn prepend_sep(url: &mut String, has_qp: bool) -> &mut String {
    if has_qp {
        *url += "&";
    } else {
        *url += "?";
    }

    url
}

/// Overwrite Modes
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverwriteMode {
    /// If a document with the specified `_key` value exists already,
    /// nothing will be done and no write operation will be carried out. The
    /// insert operation will return success in this case. This mode does not
    /// support returning the old document version using `return_old`. When using
    /// `return_new`, `None` will be returned in case the document already existed.
    Ignore,
    /// If a document with the specified `_key` value exists already,
    /// it will be patched (partially updated) with the specified document value.
    /// The `overwrite_mode` can be further controlled via the `keep_null` and
    /// `merge_objects` configuration.
    Update,
    /// If a document with the specified `_key` value exists already,
    /// it will be overwritten with the specified document value. This mode will
    /// also be used when no `overwrite_mode` is specified but the `overwrite`
    /// flag is set to true.
    Replace,
    /// If a document with the specified `_key` value exists already,
    /// return a unique constraint violation error so that the insert operation
    /// fails. This is also the default behavior in case the `overwrite_mode` is
    /// not set, and the `overwrite` flag is false or not set either.
    Conflict,
}

impl fmt::Display for OverwriteMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Ignore => "ignore",
                Self::Update => "update",
                Self::Replace => "replace",
                Self::Conflict => "conflict",
            }
        )
    }
}

impl Ser for OverwriteMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OverwriteMode::Ignore => serializer.serialize_str("ignore"),
            OverwriteMode::Update => serializer.serialize_str("update"),
            OverwriteMode::Replace => serializer.serialize_str("replace"),
            OverwriteMode::Conflict => serializer.serialize_str("conflict"),
        }
    }
}

impl<'de> Deser<'de> for OverwriteMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OverwriteModeVisitor)
    }
}

struct OverwriteModeVisitor;

impl Visitor<'_> for OverwriteModeVisitor {
    type Value = OverwriteMode;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let normalized = value.to_lowercase();
        match &normalized[..] {
            "ignore" => Ok(OverwriteMode::Ignore),
            "replace" => Ok(OverwriteMode::Replace),
            "update" => Ok(OverwriteMode::Update),
            "conflict" => Ok(OverwriteMode::Conflict),
            _ => Err(E::custom("Invalid overwrite mode")),
        }
    }
}

/// Document creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct CreateConfig<T> {
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
    document: T,
}

impl<T> CreateConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut suffix = format!("{}/{}", base, self.collection());
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(suffix, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.silent().unwrap_or(false) {
            add_qp!(suffix, has_qp, "silent=true");
        } else {
            if self.return_new().unwrap_or(false) {
                add_qp!(suffix, has_qp, "returnNew=true");
            }
            if self.return_old().unwrap_or(false) {
                add_qp!(suffix, has_qp, "returnOld=true");
            }
        }

        // Setup the overwrite related query parameters
        if let Some(mode) = self.overwrite_mode() {
            add_qp!(suffix, has_qp, &format!("overwriteMode={}", mode));

            if *mode == OverwriteMode::Update {
                if self.keep_null().unwrap_or(false) {
                    add_qp!(suffix, has_qp, "keepNull=true");
                }

                if self.merge_objects().unwrap_or(false) {
                    add_qp!(suffix, has_qp, "mergeObjects=true";);
                }
            }
        } else if self.overwrite().unwrap_or(false) {
            add_qp!(suffix, has_qp, "overwrite=true";);
        }

        suffix
    }
}

impl<T> BuildUrl for CreateConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Documents creation configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct CreatesConfig<T> {
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

impl<T> CreatesConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut suffix = format!("{}/{}", base, self.collection());
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(suffix, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.silent().unwrap_or(false) {
            add_qp!(suffix, has_qp, "silent=true");
        } else {
            if self.return_new().unwrap_or(false) {
                add_qp!(suffix, has_qp, "returnNew=true");
            }
            if self.return_old().unwrap_or(false) {
                add_qp!(suffix, has_qp, "returnOld=true");
            }
        }

        // Setup the overwrite related query parameters
        if let Some(mode) = self.overwrite_mode() {
            add_qp!(suffix, has_qp, &format!("overwriteMode={}", mode));

            if *mode == OverwriteMode::Update {
                if self.keep_null().unwrap_or(false) {
                    add_qp!(suffix, has_qp, "keepNull=true");
                }

                if self.merge_objects().unwrap_or(false) {
                    add_qp!(suffix, has_qp, "mergeObjects=true";);
                }
            }
        } else if self.overwrite().unwrap_or(false) {
            add_qp!(suffix, has_qp, "overwrite=true";);
        }

        suffix
    }
}

impl<T> BuildUrl for CreatesConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = self.build_suffix(base);
        conn.db_url()
            .join(&suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Read document configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct ReadConfig {
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

impl BuildUrl for ReadConfig {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &format!("{}/{}/{}", base, self.collection, self.key);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

impl AddHeaders for ReadConfig {
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

/// Document delete configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct DeleteConfig {
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
    /// If set to true, ignore any `_rev` attribute in the selectors. No
    /// revision check is performed. If set to false then revisions are checked.
    /// The default is true.
    #[builder(setter(into, strip_option), default)]
    ignore_revs: Option<bool>,
}

impl DeleteConfig {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/{}", base, self.collection, self.key);
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(url, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.silent().unwrap_or(false) {
            add_qp!(url, has_qp, "silent=true";);
        } else if self.return_old().unwrap_or(false) {
            add_qp!(url, has_qp, "returnOld=true";);
        }

        url
    }
}

impl AddHeaders for DeleteConfig {
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

impl BuildUrl for DeleteConfig {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Document replace configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct ReplaceConfig<T> {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The _key of the document to replace
    #[builder(setter(into))]
    key: String,
    /// The patch document
    document: T,
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
    /// If set to true, an empty object will be returned as response. No meta-data
    /// will be returned for the replaced document. This option can be used to
    /// save some network traffic.
    #[builder(setter(strip_option), default)]
    silent: Option<bool>,
    /// You can conditionally replace a document based on a target `rev` by
    /// using the `if_match` option
    #[builder(setter(into, strip_option), default)]
    if_match: Option<String>,
}

impl<T> ReplaceConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/{}", base, self.collection, self.key);
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(url, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.silent().unwrap_or(false) {
            add_qp!(url, has_qp, "silent=true");
        } else {
            if self.return_new().unwrap_or(false) {
                add_qp!(url, has_qp, "returnNew=true");
            }
            if self.return_old().unwrap_or(false) {
                add_qp!(url, has_qp, "returnOld=true");
            }
        }

        // Add ignoreRevs if necessary
        if self.ignore_revs().unwrap_or(false) {
            add_qp!(url, has_qp, "ignoreRevs=true";);
        }

        url
    }
}

impl<T> BuildUrl for ReplaceConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

impl<T> AddHeaders for ReplaceConfig<T> {
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

/// Document reads configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct ReadsConfig<T> {
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

impl<T> ReadsConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection);
        let mut has_qp = false;

        add_qp!(url, has_qp, "onlyget=true");

        // Add waitForSync if necessary
        if self.ignore_revs().unwrap_or(false) {
            add_qp!(url, has_qp, "ignoreRevs=true";);
        }

        url
    }
}

impl<T> BuildUrl for ReadsConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Document update configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct UpdateConfig<T> {
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

impl<T> UpdateConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}/{}", base, self.collection, self.key);
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(url, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.silent().unwrap_or(false) {
            add_qp!(url, has_qp, "silent=true");
        } else {
            if self.return_new().unwrap_or(false) {
                add_qp!(url, has_qp, "returnNew=true");
            }
            if self.return_old().unwrap_or(false) {
                add_qp!(url, has_qp, "returnOld=true");
            }
        }

        // Setup the overwrite related query parameters
        if self.keep_null().unwrap_or(false) {
            add_qp!(url, has_qp, "keepNull=true");
        }

        if self.merge_objects().unwrap_or(false) {
            add_qp!(url, has_qp, "mergeObjects=true");
        }

        if self.ignore_revs().unwrap_or(false) {
            add_qp!(url, has_qp, "ignoreRevs=true";);
        }

        url
    }
}

impl<T> AddHeaders for UpdateConfig<T> {
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

impl<T> BuildUrl for UpdateConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}

/// Document updates configuration
#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Serialize)]
#[getset(get = "pub(crate)")]
pub struct UpdatesConfig<T> {
    /// The collection to replace the document in
    #[builder(setter(into))]
    collection: String,
    /// The patch documents
    documents: Vec<T>,
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
}

impl<T> UpdatesConfig<T> {
    fn build_suffix(&self, base: &str) -> String {
        let mut url = format!("{}/{}", base, self.collection);
        let mut has_qp = false;

        // Add waitForSync if necessary
        if self.wait_for_sync().unwrap_or(false) {
            add_qp!(url, has_qp, "waitForSync=true");
        }

        // Setup the output related query parameters
        if self.return_new().unwrap_or(false) {
            add_qp!(url, has_qp, "returnNew=true");
        }
        if self.return_old().unwrap_or(false) {
            add_qp!(url, has_qp, "returnOld=true");
        }

        // Setup the overwrite related query parameters
        if self.keep_null().unwrap_or(false) {
            add_qp!(url, has_qp, "keepNull=true");
        }

        if self.merge_objects().unwrap_or(false) {
            add_qp!(url, has_qp, "mergeObjects=true");
        }

        if self.ignore_revs().unwrap_or(false) {
            add_qp!(url, has_qp, "ignoreRevs=true";);
        }

        url
    }
}

impl<T> BuildUrl for UpdatesConfig<T> {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url> {
        let suffix = &self.build_suffix(base);
        conn.db_url()
            .join(suffix)
            .with_context(|| format!("Unable to build '{}' url", suffix))
    }
}
