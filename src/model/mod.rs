// Copyright (c) 2021 ruarango developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Input/Output Models

use crate::{utils::prepend_sep, Connection};
use anyhow::Result;
use getset::Getters;
use reqwest::{header::HeaderMap, Url};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

pub(crate) mod auth;
pub mod coll;
pub mod common;
pub mod cursor;
pub mod db;
pub mod doc;
pub mod graph;

pub(crate) trait BuildUrl {
    fn build_url(&self, base: &str, conn: &Connection) -> Result<Url>;
}

pub(crate) trait AddHeaders {
    fn has_header(&self) -> bool;

    fn add_headers(&self) -> Result<Option<HeaderMap>>;
}

#[cfg(test)]
pub(crate) const TEST_COLL: &str = "test_coll";
#[cfg(test)]
pub(crate) const TEST_KEY: &str = "test_key";
pub(crate) const DROP_COLLECTION_QP: &str = "dropCollection=true";
pub(crate) const DROP_COLLECTION_FALSE_QP: &str = "dropCollection=false";
pub(crate) const DROP_COLLECTIONS_QP: &str = "dropCollections=true";
pub(crate) const DROP_COLLECTIONS_FALSE_QP: &str = "dropCollections=false";
pub(crate) const IGNORE_REVS_QP: &str = "ignoreRevs=true";
pub(crate) const IGNORE_REVS_FALSE_QP: &str = "ignoreRevs=false";
pub(crate) const KEEP_NULL_QP: &str = "keepNull=true";
pub(crate) const KEEP_NULL_FALSE_QP: &str = "keepNull=false";
pub(crate) const MERGE_OBJECTS_QP: &str = "mergeObjects=true";
pub(crate) const MERGE_OBJECTS_FALSE_QP: &str = "mergeObjects=false";
pub(crate) const ONLYGET_QP: &str = "onlyget=true";
pub(crate) const OVERWRITE_QP: &str = "overwrite=true";
pub(crate) const OVERWRITE_FALSE_QP: &str = "overwrite=false";
pub(crate) const OVERWRITE_MODE_QP: &str = "overwriteMode=";
pub(crate) const RETURN_NEW_QP: &str = "returnNew=true";
pub(crate) const RETURN_NEW_FALSE_QP: &str = "returnNew=false";
pub(crate) const RETURN_OLD_QP: &str = "returnOld=true";
pub(crate) const RETURN_OLD_FALSE_QP: &str = "returnOld=false";
pub(crate) const SILENT_QP: &str = "silent=true";
pub(crate) const SILENT_FALSE_QP: &str = "silent=false";
pub(crate) const WAIT_FOR_SYNC_QP: &str = "waitForSync=true";
pub(crate) const WAIT_FOR_SYNC_FALSE_QP: &str = "waitForSync=false";

#[allow(variant_size_differences)]
pub(crate) enum QueryParam {
    DropCollection(bool),
    DropCollections(bool),
    IgnoreRevs(bool),
    KeepNull(bool),
    MergeObjects(bool),
    OnlyGet,
    Overwrite(bool),
    OverwriteMode(String),
    ReturnNew(bool),
    ReturnOld(bool),
    Silent(bool),
    WaitForSync(bool),
}

impl From<QueryParam> for String {
    fn from(qp: QueryParam) -> String {
        match qp {
            QueryParam::DropCollection(v) => if v {
                DROP_COLLECTION_QP
            } else {
                DROP_COLLECTION_FALSE_QP
            }
            .to_string(),
            QueryParam::DropCollections(v) => if v {
                DROP_COLLECTIONS_QP
            } else {
                DROP_COLLECTIONS_FALSE_QP
            }
            .to_string(),
            QueryParam::IgnoreRevs(v) => if v {
                IGNORE_REVS_QP
            } else {
                IGNORE_REVS_FALSE_QP
            }
            .to_string(),
            QueryParam::KeepNull(v) => {
                if v { KEEP_NULL_QP } else { KEEP_NULL_FALSE_QP }.to_string()
            }
            QueryParam::MergeObjects(v) => if v {
                MERGE_OBJECTS_QP
            } else {
                MERGE_OBJECTS_FALSE_QP
            }
            .to_string(),
            QueryParam::OnlyGet => ONLYGET_QP.to_string(),
            QueryParam::Overwrite(v) => {
                if v { OVERWRITE_QP } else { OVERWRITE_FALSE_QP }.to_string()
            }
            QueryParam::OverwriteMode(v) => format!("{}{}", OVERWRITE_MODE_QP, v),
            QueryParam::ReturnNew(v) => if v {
                RETURN_NEW_QP
            } else {
                RETURN_NEW_FALSE_QP
            }
            .to_string(),
            QueryParam::ReturnOld(v) => if v {
                RETURN_OLD_QP
            } else {
                RETURN_OLD_FALSE_QP
            }
            .to_string(),
            QueryParam::Silent(v) => if v { SILENT_QP } else { SILENT_FALSE_QP }.to_string(),
            QueryParam::WaitForSync(v) => if v {
                WAIT_FOR_SYNC_QP
            } else {
                WAIT_FOR_SYNC_FALSE_QP
            }
            .to_string(),
        }
    }
}

pub(crate) fn add_qp<F>(opt: Option<bool>, url: &mut String, has_qp: &mut bool, f: F)
where
    F: FnOnce(bool) -> QueryParam,
{
    if let Some(flag) = opt {
        let kind = f(flag);
        let _ = prepend_sep(url, *has_qp);
        url.push_str(&String::from(kind));
        *has_qp = true;
    }
}

pub(crate) fn add_qps<F, T>(opt: Option<T>, url: &mut String, has_qp: &mut bool, f: F)
where
    F: FnOnce(String) -> QueryParam,
    T: Into<String>,
{
    if let Some(flag) = opt {
        let kind = f(flag.into());
        let _ = prepend_sep(url, *has_qp);
        url.push_str(&String::from(kind));
        *has_qp = true;
    }
}

/// Base Error Output
#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
#[getset(get = "pub")]
pub struct BaseErr {
    /// Is this an error?
    error: bool,
    /// The error code
    code: u16,
    /// The ArangoDB code
    #[serde(rename = "errorNum")]
    error_num: usize,
    /// The error message
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl fmt::Display for BaseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.error)?;
        write!(f, ", code: {}", self.code)?;
        write!(f, ", error_num: {}", self.error_num)?;
        if let Some(error_message) = &self.error_message {
            write!(f, ", error_message: {}", error_message)?;
        }
        Ok(())
    }
}
