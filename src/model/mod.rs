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
pub(crate) const ONLYGET_QP: &str = "onlyget=true";
pub(crate) const RETURN_NEW_QP: &str = "returnNew=true";
pub(crate) const RETURN_OLD_QP: &str = "returnOld=true";
pub(crate) const SILENT_QP: &str = "silent=true";
pub(crate) const WAIT_FOR_SYNC_QP: &str = "waitForSync=true";

pub(crate) enum QueryParam {
    IgnoreRevs,
    OnlyGet,
    ReturnNew,
    ReturnOld,
    Silent,
    WaitForSync,
}

impl<'a> From<QueryParam> for &'a str {
    fn from(qp: QueryParam) -> &'a str {
        match qp {
            QueryParam::IgnoreRevs => IGNORE_REVS_QP,
            QueryParam::OnlyGet => ONLYGET_QP,
            QueryParam::ReturnNew => RETURN_NEW_QP,
            QueryParam::ReturnOld => RETURN_OLD_QP,
            QueryParam::Silent => SILENT_QP,
            QueryParam::WaitForSync => WAIT_FOR_SYNC_QP,
        }
    }
}

pub(crate) fn add_qp(opt: Option<bool>, url: &mut String, has_qp: &mut bool, kind: QueryParam) {
    if opt.unwrap_or(false) {
        let _ = prepend_sep(url, *has_qp);
        url.push_str(kind.into());
        *has_qp = true;
    }
}
