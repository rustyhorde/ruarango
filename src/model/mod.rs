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
use reqwest::{header::HeaderMap, Url};

pub(crate) mod auth;
pub mod coll;
pub mod common;
pub mod db;
pub mod doc;

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
pub(crate) const IGNORE_REVS_QP: &str = "ignoreRevs=true";
pub(crate) const IGNORE_REVS_FALSE_QP: &str = "ignoreRevs=false";
pub(crate) const KEEP_NULL_QP: &str = "keepNull=true";
pub(crate) const KEEP_NULL_FALSE_QP: &str = "keepNull=false";
pub(crate) const MERGE_OBJECTS_QP: &str = "mergeObjects=true";
pub(crate) const MERGE_OBJECTS_FALSE_QP: &str = "mergeObjects=false";
pub(crate) const ONLYGET_QP: &str = "onlyget=true";
pub(crate) const RETURN_NEW_QP: &str = "returnNew=true";
pub(crate) const RETURN_NEW_FALSE_QP: &str = "returnNew=false";
pub(crate) const RETURN_OLD_QP: &str = "returnOld=true";
pub(crate) const RETURN_OLD_FALSE_QP: &str = "returnOld=false";
pub(crate) const SILENT_QP: &str = "silent=true";
pub(crate) const SILENT_FALSE_QP: &str = "silent=false";
pub(crate) const WAIT_FOR_SYNC_QP: &str = "waitForSync=true";
pub(crate) const WAIT_FOR_SYNC_FALSE_QP: &str = "waitForSync=false";

pub(crate) enum QueryParam {
    IgnoreRevs(bool),
    KeepNull(bool),
    MergeObjects(bool),
    OnlyGet,
    ReturnNew(bool),
    ReturnOld(bool),
    Silent(bool),
    WaitForSync(bool),
}

impl<'a> From<QueryParam> for &'a str {
    fn from(qp: QueryParam) -> &'a str {
        match qp {
            QueryParam::IgnoreRevs(v) => {
                if v {
                    IGNORE_REVS_QP
                } else {
                    IGNORE_REVS_FALSE_QP
                }
            }
            QueryParam::KeepNull(v) => {
                if v {
                    KEEP_NULL_QP
                } else {
                    KEEP_NULL_FALSE_QP
                }
            }
            QueryParam::MergeObjects(v) => {
                if v {
                    MERGE_OBJECTS_QP
                } else {
                    MERGE_OBJECTS_FALSE_QP
                }
            }
            QueryParam::OnlyGet => ONLYGET_QP,
            QueryParam::ReturnNew(v) => {
                if v {
                    RETURN_NEW_QP
                } else {
                    RETURN_NEW_FALSE_QP
                }
            }
            QueryParam::ReturnOld(v) => {
                if v {
                    RETURN_OLD_QP
                } else {
                    RETURN_OLD_FALSE_QP
                }
            }
            QueryParam::Silent(v) => {
                if v {
                    SILENT_QP
                } else {
                    SILENT_FALSE_QP
                }
            }
            QueryParam::WaitForSync(v) => {
                if v {
                    WAIT_FOR_SYNC_QP
                } else {
                    WAIT_FOR_SYNC_FALSE_QP
                }
            }
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
        url.push_str(kind.into());
        *has_qp = true;
    }
}
